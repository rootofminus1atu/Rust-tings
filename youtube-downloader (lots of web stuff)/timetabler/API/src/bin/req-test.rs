use tracing::info;
use tracing_subscriber;


type TypicalRes = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> TypicalRes {
    tracing_subscriber::fmt().init();

    // let mut res = reqwest::get("https://tenor.com/view/sashley-gif-21971452")
    //     .await?
    //     .text()
    //     .await?;
    // res.truncate(res.len() - 16000);
    // info!("{}", res);

    Ok(())
}