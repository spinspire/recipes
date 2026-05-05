use lazy_static::lazy_static;

use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::time::Instant;

use axum::extract::Multipart;
use axum::response::{IntoResponse, Result};
use axum::{Extension, Json};
use csv::StringRecord;
use rusqlite::{params_from_iter, Connection};
use serde::Serialize;

use crate::db::Database;

#[derive(Serialize)]
pub struct File {
    file_name: String,
    content_type: String,
    size: Option<usize>,
}

pub async fn root(
    Extension(database): Extension<Database<'static>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut files: Vec<File> = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        // handle only file fields, skip others
        if let Some(_) = field.file_name() {
            let timer = Instant::now();
            let file = File {
                file_name: field.file_name().unwrap().to_string(),
                content_type: field.content_type().unwrap().to_string(),
                size: None,
            };
            let table_name = make_table_name(&file.file_name);
            files.push(file);
            // this call (bytes().await...) should be LAST one
            // or else you'll get "moved" compiler error
            // process1(field.bytes().await.unwrap());
            let db = database.connection();
            db.execute_batch("PRAGMA synchronous = OFF;PRAGMA joural_mod = WAL;")
                .unwrap();
            println!("TABLE_NAME:{}", table_name);
            ingest(db, &table_name, field.text().await.unwrap()).unwrap();
            println!("TOTAL: {:?}", timer.elapsed());
        }
    }

    // Ok(Json(HashMap::from([("message", "hi")])))
    Ok(Json(files))
}

lazy_static! {
    static ref RE_EXTENSION: Regex = Regex::new(r"\..*$").unwrap();
    static ref RE_SPECIAL_CHARS: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    static ref RE_REPEATED_UNDERSCORE: Regex = Regex::new(r"__+").unwrap();
    static ref RE_UNDERSCORES_AT_EDGES: Regex = Regex::new(r"^_+|_+$").unwrap();
    static ref RE_LEADING_NONALPHA: Regex = Regex::new(r"^[[:^alpha:]]").unwrap();
}

fn make_table_name(file_name: &String) -> String {
    // strip extension
    let base_name = RE_EXTENSION.replace_all(file_name, "");
    // sanitize the rest
    sanitize(&base_name)
}

fn sanitize(inp: &str) -> String {
    // replace all special-chars with "_"
    let out1 = RE_SPECIAL_CHARS.replace_all(inp, "_");
    // reduce all repeated "_" to one
    let out2 = RE_REPEATED_UNDERSCORE.replace_all(&out1, "_");
    // remove all "_" at start/end
    let out3 = RE_UNDERSCORES_AT_EDGES.replace_all(&out2, "");
    // starts with non-alpha, remove it
    let out4 = RE_LEADING_NONALPHA.replace_all(&out3, "");
    if out4.len() < 1 {
        // nothing left? return a new random string
        return random_string(10).to_lowercase();
    }
    out4.to_lowercase()
}

fn random_string(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

fn ingest(db: Connection, table_name: &str, text: String) -> Result<usize, rusqlite::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(text.as_bytes());
    let mut i = 0;
    let mut cols: Vec<String> = vec![];
    for r in reader.records() {
        match r {
            Ok(record) => {
                if i == 0 {
                    for c in record.iter() {
                        cols.push(c.to_string());
                    }
                    create_table(&db, table_name, &cols)?;
                } else {
                    insert_row(&db, table_name, &cols, &record, i == 1)?;
                }
                i = i + 1;
            }
            Err(e) => println!("Err: {}", e),
        }
    }
    println!("SUMMARY: count={}", i);
    Ok(i)
}

fn create_table(db: &Connection, name: &str, columns: &Vec<String>) -> Result<(), rusqlite::Error> {
    let sql = format!("DROP TABLE IF EXISTS \"{}\"", name);
    println!("SQL: {}", sql);
    db.execute(&sql, [])?;
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" (ROWID INTEGER PRIMARY KEY AUTOINCREMENT)",
        name
    );
    println!("SQL: {}", sql);
    db.execute(&sql, [])?;
    for c in columns {
        let sql = format!("ALTER TABLE {} ADD COLUMN \"{}\" TEXT", name, c);
        println!("SQL: {}", sql);
        db.execute(&sql, [])?;
    }
    Ok(())
}

fn insert_row(
    db: &Connection,
    table: &str,
    columns: &Vec<String>,
    record: &StringRecord,
    log: bool,
) -> Result<(), rusqlite::Error> {
    let sql = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table,
        {
            columns
                .iter()
                .map(|c| format!("\"{}\"", c))
                .collect::<Vec<_>>()
                .join(",")
        },
        {
            columns
                .iter()
                .enumerate()
                .map(|(i, _)| format!("?{}", i + 1))
                .collect::<Vec<_>>()
                .join(",")
        }
    );
    if log {
        println!("SQL: {}", sql);
    }
    db.execute(&sql, params_from_iter(record.iter()))?;
    Ok(())
}
