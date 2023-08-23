use std::error::Error;
use std::fs;
use rustube::Video;
use tokio;
use url::Url;

use rustube::video_info::player_response::streaming_data::QualityLabel;


use serde_json::json;
use serde_json::Value;

fn parse_quality_label(label: &str) -> Result<QualityLabel, &'static str> {

    let json_value: Value = json!(label);

    println!("json_value: {:?}", json_value);

    let parsed: Result<QualityLabel, _> = serde_json::from_value(json_value);

    println!("parsed: {:?}", parsed);

    
    parsed.or(Err("Invalid quality label"))

}

macro_rules! map_quality_label {
    ($label:expr, $($variant:ident),*) => {
        match $label {
            $(stringify!($variant) => Some(QualityLabel::$variant),)*
            _ => None,
        }
    };
}

#[tokio::main]
async fn main() {
    /* 
    let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
    println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
    */
    


    let vid = match get_vid_from_url_str("https://www.youtube.com/watch?v=CYtZKmm0Poo").await {
        Ok(vid) => vid,
        Err(e) => panic!("Error getting video: {}", e),
    };

    let stream = vid
        .streams()
        .iter()
        .filter(|s| s.includes_audio_track && s.includes_video_track)
        .max_by_key(|s| s.quality_label)
        .unwrap();

    // before downloading check if there is a vid called `stream.video_details.video_id.mp4 in the` `downloads` folder

    let directory_path = "downloads";
    let video_id = &stream.video_details.video_id; // Replace with the actual video ID
    let video_file_name = format!("{}/{}.mp4", directory_path, video_id);

    if fs::metadata(&video_file_name).is_ok() {
        println!("Video file exists: {}", video_file_name);
        // return that video file

    } else {
        println!("Video file does not exist: {}", video_file_name);
        stream.download_to_dir("./downloads").await.unwrap();
    }

    
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
