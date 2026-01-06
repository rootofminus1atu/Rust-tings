use tokio::sync::mpsc::Sender;
use std::future::Future;
use async_trait::async_trait;

#[derive(Debug)]
struct Event<T>(T);

#[async_trait]
trait Producer {
    type Output;
    async fn produce(&mut self) -> Event<Self::Output>;
}

#[async_trait]
trait SourceRunner {
    async fn run<P: Producer>(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>);
}

struct DefaultSourceRunner;
#[async_trait]
impl SourceRunner for DefaultSourceRunner {
    async fn run<P: Producer>(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>) {
        loop {
            let event = producer.produce().await;
            sender.send(event).await.unwrap();
        }
    }
}

struct Source<P: Producer, R: SourceRunner> {
    producer: P,
    runner: R,
    sender: Sender<Event<P::Output>>,
}

impl<P, R> Source<P, R> 
where 
    P: Producer,
    R: SourceRunner,
{
    fn new(producer: P, runner: R, sender: Sender<Event<P::Output>>) -> Self {
        Self { producer, runner, sender }
    }
}

impl<P> Source<P, DefaultSourceRunner>
where
    P: Producer,
{
    fn with_default_runner(producer: P, sender: Sender<Event<P::Output>>) -> Self {
        Self {
            producer,
            runner: DefaultSourceRunner,
            sender,
        }
    }
}

#[async_trait]
trait Runnable {
    async fn run(&mut self);
}

#[async_trait]
trait RunnableObj {
    async fn run(&mut self);
}

#[async_trait]
impl<T: Runnable> RunnableObj for T {
    async fn run(&mut self) {
        Runnable::run(self).await
    }
}

fn make_runnable_object<T>(thing: T) -> Box<dyn RunnableObj>
where
    T: Runnable + 'static,
{
    Box::new(thing)
}

#[async_trait]
impl<P, R> Runnable for Source<P, R>
where
    P: Producer + Send,
    R: SourceRunner + Send,
{
    async fn run(&mut self) {
        self.runner.run(&mut self.producer, self.sender.clone()).await;
    }
}

struct LogProducer {
    config: String,
    counter: i32,
}

#[async_trait]
impl Producer for LogProducer {
    type Output = (String, i32);

    async fn produce(&mut self) -> Event<Self::Output> {
        self.counter += 1;
        Event(("new event".to_string(), self.counter))
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let lp = LogProducer { config: "config".into(), counter: 0 };
    let ls = Source::with_default_runner(lp, tx);

    let mut nodes: Vec<Box<dyn RunnableObj>> = vec![make_runnable_object(ls)];

    // Run all nodes
    for node in &mut nodes {
        tokio::spawn(async move {
            node.run().await;
        });
    }

    while let Some(event) = rx.recv().await {
        println!("Received event: {:?}", event);
    }
}
