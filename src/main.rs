#[macro_use] extern crate rocket;
use rocket::fs::TempFile;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/download")]
fn download() -> &'static str {
    "Hello, world!"
}

#[post("/upload", format = "plain", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    file.persist_to(permanent_location).await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, download, upload])
}