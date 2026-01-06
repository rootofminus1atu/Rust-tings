use tokio::sync::mpsc::{Sender, Receiver};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    println!("Hello, world!");


    Ok(())
}

#[derive(Debug, Clone)]
struct Event(String);
// <D: Clone> {
//     data: D
// }

pub struct Poller {
    source: Source
}
impl Runner for Poller {
    async fn run<S: Source>(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let e = 
        }
    }
}

trait Runner {
    async fn run(&self);
}

trait Source<R: Runner> {
    async fn produce(&self) -> Event;

    async fn run(&self) {
        loop {
            let e = self.produce().await;
            println!("sending event")
        }
    }
}

struct LogWatcher {
    config: String
}

impl Source for LogWatcher {
    async fn produce(&self) -> Event {
        return Event("new event".to_string());
    }

    // if not provided in the generic arg then they have to implement run here
}

impl Source<Poller> for LogWatcher {
    async fn produce(&self) -> Event {
        return Event("new event".to_string());
    }

    // no need to implemenet the run anymore
}
