use tokio; // Make sure you have tokio in your dependencies in Cargo.toml
use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use youtube_dl::YoutubeDl;
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=VFbhKZFzbzk")
.download_to(".");

    println!("{:?}", output);

    Ok(())
}
