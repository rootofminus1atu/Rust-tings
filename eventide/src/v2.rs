use std::future::Future;

use tokio::sync::mpsc::{Sender, Receiver};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    println!("Hello, world!");

    // pseudocode idea, so that creating these topologies is user friendly

    // sources transforms sinks created

    // let mut topology = Topology::new();

    // topology.add_edge(s1, t1);
    // topology.add_edge(s1, t2);
    // topology.add_edge(t1, sink);
    // topology.add_edge(t2, sink);

    // topology.start().await;

    let (tx, rx) = tokio::sync::mpsc::channel(32);

    let lp = LogProducer { config: "config".into(), counter: 0 };


    let ls = Source::with_default_runner(lp, tx);

    let mut nodes = vec![make_runnable_object(ls)];

    Ok(())
}

#[derive(Debug)]
struct Event<T>(T);


trait Producer {
    type Output: Send;
    async fn produce(&mut self) -> Event<Self::Output>;
}

trait SourceRunner {
    async fn run<P: Producer>(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>);
}


struct DefaultSourceRunner;
impl SourceRunner for DefaultSourceRunner {
    async fn run<P: Producer>(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>) {
        loop {
            let event = producer.produce().await;
            sender.send(event).await;
            // TODO: error handle stuff
        }
    }
}
// trait SourceRunner2<P: Producer> {
//     async fn run(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>);
// }
// impl<P: Producer> SourceRunner2<P> for DefaultSourceRunner {
//     async fn run(&mut self, producer: &mut P, sender: Sender<Event<P::Output>>) {
//         todo!()
//     }
// }

struct Source<P: Producer, R: SourceRunner> {
    producer: P,
    runner: R,
    sender: Sender<Event<P::Output>>
}
impl<P, R> Source<P, R> 
where 
    P: Producer,
    R: SourceRunner
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

type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'static>>;

trait Runnable {
    fn run(&mut self) -> BoxFuture<()>;
}

#[async_trait::async_trait]
trait RunnableObj {
    async fn run(&mut self);
}

#[async_trait::async_trait]
impl<T: Runnable + Send> RunnableObj for T {
    async fn run(&mut self) {
        Runnable::run(self).await
    }
}

fn make_runnable_object<T>(thing: T) -> Box<dyn RunnableObj>
where
    T: Runnable + Send + 'static,
{
    Box::new(thing)
}

#[async_trait::async_trait]
impl<P, R> Runnable for Source<P, R>
where
    P: Producer + Send,
    R: SourceRunner + Send,
{
    async fn run(&mut self) {
        self.runner.run(&mut self.producer, self.sender.clone()).await;
    }
}


// impl<P, R> Runnable for Source<P, R> 
// where
//     P: Producer + Send,
//     R: SourceRunner + Send,
// {
//     fn run(&mut self) -> BoxFuture<()> {
//         // TODO: clone? yes or no?
//         let fut = self.runner.run(&mut self.producer, self.sender.clone());
//         Box::pin(async move {
//             fut.await;
//         })
//     }
// }

struct LogProducer {
    config: String,
    counter: i32
}
impl Producer for LogProducer {
    type Output = (String, i32);

    async fn produce(&mut self) -> Event<Self::Output> {
        self.counter += 1;
        return Event(("new event".to_string(), self.counter));
    }
}





trait Source2 {
    type Output;

    async fn produce(&mut self) -> Event<Self::Output>;

    async fn run(&mut self, sender: Sender<Event<Self::Output>>) {
        loop {
            let e = self.produce().await;
            // TODO: handle errors
            sender.send(e).await;
        }
    }
}

struct LogWatcher {
    config: String,
    counter: i32
}
impl Source2 for LogWatcher {
    type Output = (String, i32);

    async fn produce(&mut self) -> Event<(String, i32)> {
        self.counter += 1;
        return Event(("new event".to_string(), self.counter));
    }
}


trait Transform {
    type Input;
    type Output;

    async fn process(&mut self, input: Event<Self::Input>) -> Event<Self::Output>;

    // might need custom runners too, for exapmle the user might want:
    // 1. sequential mode, waiting with processing until next is done
    // 2. immediate mode, processing as fast as possible in parallel, not caring abour order
    // 3. hybrid mode, processing in parallel but keeping the order anyway
    // 4. others? 
    async fn run(&mut self, mut receiver: Receiver<Event<Self::Input>>, sender: Sender<Event<Self::Output>>) {
        while let Some(e) = receiver.recv().await {
            let e = self.process(e).await;
            sender.send(e).await;
        }
    }
}

struct LogTransformer {
    config: String
}
impl Transform for LogTransformer {
    type Input = (String, i32);
    type Output = i32;

    async fn process(&mut self, input: Event<(String, i32)>) -> Event<i32> {
        // doing some processing
        return Event(input.0.1)
    }
}

trait Sink {
    type Input;

    async fn consume(&mut self, input: Event<Self::Input>);

    async fn run(&mut self, mut receiver: Receiver<Event<Self::Input>>) {
        while let Some(e) = receiver.recv().await {
            self.consume(e).await;
        }
    }
}

struct LogDisplayer {
    config: String
}
impl Sink for LogDisplayer {
    type Input = i32;

    async fn consume(&mut self, input: Event<i32>) {
        todo!()
    }
}

// struct Topology<I1, I2, S: Source<I1>, T: Transform<I1, I2>, K: Sink<I2>> {
//     source: S,
//     transforms: Vec<T>,
//     sink: K
// }

// impl Topology {
//     pub fn new() -> Self {}

//     pub fn start()
// }

// type BoxFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'static>>;


// struct Topology<S, O> {
//     source: S,
//     steps: Vec<Box<dyn Step<O>>>
// }

// trait Step<I> {
//     fn run(self: Box<Self>, input: Receiver<Event<I>>) -> BoxFuture<()>;
// }

// impl<S, O> Topology<S, O>
// where 
//     S: Source<O>
// {
//     pub fn new(source: S) -> Self {
//         Self {
//             source,
//             steps: Vec::new()
//         }
//     }

//     pub fn add_transform<T, N>(mut self, transform: T) -> Topology<S, N>
//     where 
//         T: Transform<O, N>
//     {

//     }
// }

// struct Topology<S, O> {
//     source: S,
//     steps: Vec<Box<dyn Step<O>>>,
// }

// trait Step<I>: Send {
//     fn run(self: Box<Self>, input: Receiver<Event<I>>) -> BoxFuture<'static, ()>;
// }

// type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

// impl<S, O> Topology<S, O>
// where
//     S: Source<O>,
//     O: Send + 'static,
// {
//     pub fn new(source: S) -> Self {
//         Self {
//             source,
//             steps: Vec::new(),
//         }
//     }

//     pub fn add_transform<T, N>(mut self, transform: T) -> Topology<S, N>
//     where
//         T: Transform<O, N> + 'static,
//         N: Send + 'static,
//         O: Send + 'static,
//     {
//         self.steps.push(Box::new(TransformStep { transform }));
//         Topology {
//             source: self.source,
//             steps: self.steps,
//         }
//     }

//     pub fn add_sink<K>(mut self, sink: K) -> CompleteTopology<S, O, K>
//     where
//         K: Sink<O> + 'static,
//     {
//         CompleteTopology {
//             source: self.source,
//             steps: self.steps,
//             sink,
//         }
//     }
// }

// struct TransformStep<T, I, O> {
//     transform: T,
// }

// impl<T, I, O> Step<I> for TransformStep<T, I, O>
// where
//     T: Transform<I, O> + 'static,
//     I: Send + 'static,
//     O: Send + 'static,
// {
//     fn run(self: Box<Self>, input: Receiver<Event<I>>) -> BoxFuture<'static, ()> {
//         let (tx, rx) = channel(16);
//         let fut = async move {
//             self.transform.run(input, tx).await;
//         };
//         Box::pin(fut)
//     }
// }

// struct CompleteTopology<S, O, K> {
//     source: S,
//     steps: Vec<Box<dyn Step<O>>>,
//     sink: K,
// }

// impl<S, O, K> CompleteTopology<S, O, K>
// where
//     S: Source<O> + Send + 'static,
//     K: Sink<O> + Send + 'static,
//     O: Send + 'static,
// {
//     pub async fn start(self) {
//         let (tx, rx) = channel(16);
//         let source_handle = tokio::spawn(self.source.run(tx));
//         let mut prev_rx = rx;

//         // Run all steps (transforms)
//         for step in self.steps {
//             let fut = step.run(prev_rx);
//             prev_rx = /* get the output receiver from the step */;
//             tokio::spawn(fut);
//         }

//         // Sink
//         let sink_handle = tokio::spawn(self.sink.run(prev_rx));

//         let _ = tokio::join!(source_handle, sink_handle);
//     }
// }