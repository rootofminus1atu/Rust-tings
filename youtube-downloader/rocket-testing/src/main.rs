#[macro_use] extern crate rocket;

#[allow(unused_imports)]
use std::path::{PathBuf, Path};

#[allow(unused_imports)]
use rocket::fs::{NamedFile, relative};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// method for sending the video vid.mp4 to the frontend
#[get("/download")]
async fn download() -> Option<NamedFile> {
    NamedFile::open("static/vid.mp4").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, download])
}
