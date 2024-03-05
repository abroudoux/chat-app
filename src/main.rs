#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

mod routes;
mod message;

use rocket::tokio::sync::broadcast::channel;
use rocket::fs::{relative, FileServer};

use routes::{events, post};
use message::Message;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}