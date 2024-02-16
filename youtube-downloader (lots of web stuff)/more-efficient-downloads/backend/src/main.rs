#[macro_use] extern crate rocket;

use rocket::{get, response::stream::ByteStream, Response};
use ureq; // Using the ureq crate for synchronous HTTP requests

struct VideoStream {
    video_url: String,
}

impl<'r> rocket::response::Responder<'r, 'static> for VideoStream {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        // Make an HTTP request to fetch the video content using ureq
        let video_response = ureq::get(&self.video_url).call();
        if video_response.ok() {
            let video_bytes = video_response.into_reader();

            // Create a streaming response
            Ok(Response::build()
                .header(rocket::http::ContentType::new("video", "mp4"))
                .streamed_body(ByteStream::new(video_bytes))
                .finalize())
        } else {
            // Handle error response
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/download/<video_id>")]
fn download(video_id: String) -> VideoStream {
    let video_url = format!("https://www.youtube.com/watch?v={}", video_id);
    VideoStream { video_url }
}

fn main() {
    rocket::build().mount("/", routes![download]).launch();
}
