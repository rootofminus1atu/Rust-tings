#[macro_use] extern crate rocket;

#[allow(unused_imports)]
use {
    rustube,
    std::path::{PathBuf, Path},
    rocket::fs::{NamedFile, relative},
    rocket::serde::{Deserialize, Serialize, json::Json}
};



#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

// method for sending the video vid.mp4 to the frontend
#[allow(dead_code)]
#[get("/downloadtest")]
async fn downloadtest() -> Option<NamedFile> {
    NamedFile::open("static/vid.mp4").await.ok()
}




#[derive(Debug)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DownloadData {
    url: String
}


// method for downloading a video and then allowing the user to download it from the frontend
#[post("/download", data = "<data>")]
fn download(data: Json<DownloadData>)  {
    println!("data: {:?}", data);
    let url_str = data.url.clone();
    println!("data: {:?}", url_str);
    
}


#[get("/testlol")]
fn testlol() -> &'static str {
    println!("I was called lol");
    "This is a test route"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, download, testlol])
}



/*
https://twitter.com/DarrenBaldwin03/status/1682028733736288258
https://twitter.com/DarrenBaldwin03/status/1682028733736288258

*/
