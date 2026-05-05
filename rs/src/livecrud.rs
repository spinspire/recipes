use axum::{response::IntoResponse, Extension};
use axum_live_view::{
    event_data::EventData, html, live_view::Updated, Html, LiveView, LiveViewUpgrade,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{remove, save, Database},
    User,
};

// Our handler function for `GET /`
pub async fn root(
    live: LiveViewUpgrade,
    Extension(database): Extension<Database<'static>>,
) -> impl IntoResponse {
    let users = match crate::db::list(database.connection()) {
        Ok(users) => users,
        Err(_) => vec![],
    };
    let list = CrudList {
        users: users,
        database: database,
    };

    live.response(|embed_live_view| {
        html! {
            <!DOCTYPE html>
            <html>
                <head>
                <link
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.min.css"
                />
                </head>
                <body>
                    <form axm-submit={Msg::Add}>
                        <input placeholder="username" name="username"/>
                        <button type="submit">"Add"</button>
                    </form>
                    <button axm-click={Msg::Load}>"Reload"</button>
                    { embed_live_view.embed(list) }
                    <script src="/live/live-view.js"></script>
                </body>
            </html>
        }
    })
}

// Our live view is just a regular Rust struct...
struct CrudList {
    users: Vec<User>,
    database: Database<'static>,
}

impl CrudList {
    fn load(mut self) -> Self {
        match crate::db::list(self.database.connection()) {
            Ok(users) => {
                println!("{}", users.len());
                self.users = users
            }
            Err(_) => self.users.clear(),
        }
        self
    }

    fn add(self, user: &User) -> Self {
        _ = save(self.database.connection(), &user);
        self.load()
    }

    fn remove(self, user: &User) -> Self {
        _ = remove(self.database.connection(), user.id.unwrap());
        self.load()
    }
}

// ...that implements the `LiveView` trait.
impl LiveView for CrudList {
    // This is the type of update messages our HTML contains. They will be sent
    // to the view in the `update` method
    type Message = Msg;

    // Update the view based on which message it receives.
    //
    // `EventData` contains data from the event that happened in the
    // browser. This might be values of input fields or which key was pressed in
    // a keyboard event.
    fn update(mut self, msg: Msg, data: Option<EventData>) -> Updated<Self> {
        self = match msg {
            Msg::Load => self.load(),
            Msg::Add => {
                let values: User = data.unwrap().as_form().unwrap().deserialize().unwrap();
                self.add(&values)
            }
            Msg::Delete => {
                let values: User = data.unwrap().as_form().unwrap().deserialize().unwrap();
                self.remove(&values)
            }
        };
        Updated::new(self)
    }

    // Render the live view into an HTML template. This function is called during
    // the initial render in `LiveViewManager::embed` and for each subsequent
    // update.
    //
    // The HTML is diff'ed on the server and only minimal deltas are sent over
    // the wire. The browser then builds the full HTML template and efficiently
    // updates the DOM.
    fn render(&self) -> Html<Self::Message> {
        html! {
            <table>
                <thead>
                    <tr>
                        <th>"&times;"</th>
                        <th>"id"</th>
                        <th>"username"</th>
                    </tr>
                </thead>
                <tbody>
                for user in &self.users {
                    <tr>
                        <td>
                            <form axm-submit={Msg::Delete}>
                                <input name="id" type="hidden" value={user.id.unwrap()}/>
                                <input name="username" type="hidden" value=""/>
                                <button type="submit">"&times;"</button>
                            </form>
                        </td>
                        <td>{user.id.unwrap()}</td>
                        <td>{user.username.to_string()}</td>
                    </tr>
                }
                </tbody>
            </table>
        }
    }

    // The `LiveView` trait also has a `mount` method that is called when a new
    // WebSocket connects. This can be used to perform auth, load data that
    // isn't needed for the first response, and spawn a task that can send
    // messages to the view itself from other parts of the application.
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Msg {
    Load,
    Add,
    Delete,
}
