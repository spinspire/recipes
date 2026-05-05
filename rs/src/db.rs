use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Extension;
use axum::Json;
use rusqlite::Connection;
use rusqlite::Error;
use serde::Serialize;

use crate::User;

// credit: https://docs.rs/axum-sqlite/latest/axum_sqlite/
#[derive(Clone)]
pub struct Database<'a> {
    path: &'a str,
}

impl<'a> Database<'a> {
    pub fn new(path: &'a str) -> Extension<Self> {
        let ext = Extension(Self { path });
        ext.connection()
            .execute(
                "CREATE TABLE IF NOT EXISTS users (username TEXT NOT NULL);",
                [],
            )
            .unwrap();
        ext
    }
    pub fn connection(&self) -> Connection {
        Connection::open(self.path).unwrap()
    }
}

pub fn db_result2json<T>(result: Result<T, rusqlite::Error>) -> Response
where
    T: Serialize,
{
    match result {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            println!("ERROR:{:?}", e);
            match e {
                rusqlite::Error::QueryReturnedNoRows => (StatusCode::NOT_FOUND, "Not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error"),
            }
        }
        .into_response(),
    }
}

pub fn save(connection: Connection, user: &User) -> Result<User, Error> {
    match user.id {
        Some(_) => update(connection, user),
        None => insert(connection, user),
    }
}

fn insert(connection: Connection, user: &User) -> Result<User, Error> {
    connection.execute(
        "INSERT INTO users (username) VALUES (?1)",
        &[&user.username.to_string()],
    )?;
    Ok(User {
        id: Some(connection.last_insert_rowid()),
        username: user.username.to_string(),
    })
}

fn update(connection: Connection, user: &User) -> Result<User, Error> {
    let id = user.id.unwrap();
    connection.execute(
        "UPDATE users SET username=?1 WHERE rowid = ?2",
        &[&user.username.to_string(), &id.to_string()],
    )?;
    Ok(User {
        id: user.id,
        username: user.username.to_string(),
    })
}

pub fn list(connection: Connection) -> Result<Vec<User>, Error> {
    let mut st = connection.prepare("SELECT rowid,* FROM users")?;
    let mut result: Vec<User> = Vec::new();
    let mut rows = st.query([])?;
    while let Some(row) = rows.next()? {
        result.push(User {
            id: row.get(0)?,
            username: (row.get(1)?),
        })
    }
    // std::thread::sleep(Duration::from_secs(10));
    Ok(result)
}

pub fn one(connection: Connection, id: i64) -> Result<User, Error> {
    connection.query_row("SELECT rowid,* FROM users WHERE rowid=?1", &[&id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
        })
    })
}

pub fn remove(connection: Connection, id: i64) -> Result<usize, Error> {
    connection.execute("DELETE FROM users WHERE rowid=?1", &[&id])
}
