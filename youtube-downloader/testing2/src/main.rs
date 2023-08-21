
use rustube;
use rustube::blocking::Video;
use rustube::Id;


fn main() {
    // simple_download();
    let id_res = Id::from_str("CYtZKmm0Poo");

    let id = match id_res {
        Ok(id) => id,
        Err(e) => panic!("Error parsing ID: {}", e),
    };

    let video = match Video::from_id(id) {
        Ok(video) => video,
        Err(e) => panic!("Error getting video: {}", e),
    };
    /* 
    let best_quality = video
        .streams()
        .iter()
        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
        .max_by_key(|stream| stream.quality_label);

    let stream = match best_quality {
        Some(stream) => stream,
        None => panic!("No streams found"),
    };
    println!("Downloading video with quality: {:?}", stream.quality_label);*/

    for stream in video.streams().iter().filter(|stream| stream.includes_video_track && stream.includes_audio_track) {
        println!("Stream quality things: {:?}", (stream.quality_label, stream.quality, stream.includes_audio_track, stream.includes_video_track));
    }

    println!("Title: {}", &video.title());
}



#[allow(dead_code)]
fn simple_download() {
    println!("Starting download of video");

    let url = "https://youtu.be/59UcmZGGzG8";
    let path_to_video = rustube::blocking::download_best_quality(url);

    match path_to_video {
        Ok(path) => println!("Downloaded video to {}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}