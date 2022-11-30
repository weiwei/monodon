#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::data::{Data, ToByteUnit};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CACHE_DIR: PathBuf = [env::current_dir().unwrap().to_str().unwrap(), "cache"]
        .iter()
        .collect::<PathBuf>();
}

#[get("/")]
fn index() -> &'static str {
    CACHE_DIR.to_str().unwrap()
}

#[get("/file_exists/<file_name>")]
fn file_exists(file_name: &str) -> Status {
    // "Hello, world!"
    let mut new_dir = CACHE_DIR.clone();
    new_dir.push(file_name);
    if new_dir.exists() {
        return Status::Ok;
    } else {
        return Status::NotFound;
    }
}

#[get("/download/<file_name>")]
async fn download(file_name: &str) -> NamedFile {
    let mut new_dir = CACHE_DIR.clone();
    new_dir.push(file_name);
    let file = NamedFile::open(new_dir).await.unwrap();
    file
}
#[post("/upload/<file_name>", format = "plain", data = "<data>")]
async fn upload(file_name: &str, mut data: Data<'_>) -> Status {
    let mut new_dir = CACHE_DIR.clone();
    new_dir.push(file_name);
    let mut file = File::create(new_dir);
    data.open(512.kibibytes()).stream_to(file).await;
    return Status::Ok;
}

#[launch]
fn rocket() -> _ {
    if !CACHE_DIR.exists() {
        std::fs::create_dir(CACHE_DIR.as_path()).unwrap();
    }
    rocket::build().mount("/", routes![index, download, file_exists])
}
