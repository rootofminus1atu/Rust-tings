use std::error::Error;

use rustube::Video;
use rustube::Id;
use tokio;
use url::Url;

#[tokio::main]
async fn main() {
    /* 
    let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
    println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
    */
    let id_res = Id::from_str("CYtZKmm0Poo");

    let id = match id_res {
        Ok(id) => id,
        Err(e) => panic!("Error parsing ID: {}", e),
    };

    let video = match Video::from_id(id).await {
        Ok(video) => video,
        Err(e) => panic!("Error getting video: {}", e),
    };

    let streams: Vec<&rustube::Stream> = video
        .streams()
        .iter()
        .filter(|s| s.includes_audio_track && s.includes_video_track)
        .collect();

    for stream in streams {
        println!("Stream: {:?}", (stream.quality, stream.quality_label));
    }


    let vid = match get_vid_from_url_str("https://www.youtube.com/watch?v=CYtZKmm0Poo").await {
        Ok(vid) => vid,
        Err(e) => panic!("Error getting video: {}", e),
    };

    let strms = get_valid_streams(&vid);




}


async fn get_vid_from_url_str(url: &str) -> Result<Video, Box<dyn std::error::Error>> {
    let url: Url = Url::parse(url)?;
    let video = Video::from_url(&url).await?;
    
    Ok(video)
}

fn get_valid_streams(video: &Video) -> Vec<&rustube::Stream> {
    let streams: Vec<&rustube::Stream> = video
        .streams()
        .iter()
        .filter(|s| s.includes_audio_track && s.includes_video_track)
        .collect();
    
    streams
}
