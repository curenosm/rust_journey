#[macro_use] extern crate rocket;

use std::io;
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::{spawn_blocking};

#[get("/")]
fn index() -> &'static str {
   "Hello, World!" 
}

#[get("/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hi", routes![index])
        .mount("/tasks", routes![blocking_task])
        .mount("/delay", routes![delay])
}

