use std::path::{Path, PathBuf};
use std::time::Duration;

use rocket::fs::{FileServer, NamedFile};
use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::tokio::time::sleep;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")] // <- route attribute
fn world() -> &'static str {
    // <- request handler
    "hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

// file server example,
// prefer using `.mount("/public", FileServer::from("static/"))`
#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("User ID (usize): {}", id)
}
#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!("User ID (isize): {}", id)
}
#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!("User ID (string): {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, world, delay, hello, files])
        .mount("/public", FileServer::from("static/")) // suggested way to serve static files
        .mount("/", routes![user, user_int, user_str])
}
