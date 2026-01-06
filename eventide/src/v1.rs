use tokio::sync::mpsc::{Sender, Receiver};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    println!("Hello, world!");

    
    let lw = LogWatcher { config: "config".into(), counter: 0 };



    Ok(())
}

#[derive(Debug)]
struct Event<T>(T);

trait Source<O> {
    async fn produce(&mut self) -> Event<O>;

    async fn run(&mut self, sender: Sender<Event<O>>) {
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
impl Source<(String, i32)> for LogWatcher {
    async fn produce(&mut self) -> Event<(String, i32)> {
        self.counter += 1;
        return Event(("new event".to_string(), self.counter));
    }
}


trait Transform<I, O> {
    async fn process(&mut self, input: Event<I>) -> Event<O>;

    // might need custom runners too, for exapmle the user might want:
    // 1. sequential mode, waiting with processing until next is done
    // 2. immediate mode, processing as fast as possible in parallel, not caring abour order
    // 3. hybrid mode, processing in parallel but keeping the order anyway
    // 4. others? 
    async fn run(&mut self, mut receiver: Receiver<Event<I>>, sender: Sender<Event<O>>) {
        while let Some(e) = receiver.recv().await {
            let e = self.process(e).await;
            sender.send(e).await;
        }
    }
}

struct LogTransformer {
    config: String
}
impl Transform<(String, i32), i32> for LogTransformer {
    async fn process(&mut self, input: Event<(String, i32)>) -> Event<i32> {
        // doing some processing
        return Event(input.0.1)
    }
}

trait Sink<I> {
    async fn consume(&mut self, input: Event<I>);

    async fn run(&mut self, mut receiver: Receiver<Event<I>>) {
        while let Some(e) = receiver.recv().await {
            self.consume(e).await;
        }
    }
}

struct LogDisplayer {
    config: String
}
impl Sink<i32> for LogDisplayer {
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

struct Topology<S, O> {
    source: S,
    steps: Vec<Box<dyn Step<O>>>,
}

trait Step<I>: Send {
    fn run(self: Box<Self>, input: Receiver<Event<I>>) -> BoxFuture<'static, ()>;
}

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S, O> Topology<S, O>
where
    S: Source<O>,
    O: Send + 'static,
{
    pub fn new(source: S) -> Self {
        Self {
            source,
            steps: Vec::new(),
        }
    }

    pub fn add_transform<T, N>(mut self, transform: T) -> Topology<S, N>
    where
        T: Transform<O, N> + 'static,
        N: Send + 'static,
        O: Send + 'static,
    {
        self.steps.push(Box::new(TransformStep { transform }));
        Topology {
            source: self.source,
            steps: self.steps,
        }
    }

    pub fn add_sink<K>(mut self, sink: K) -> CompleteTopology<S, O, K>
    where
        K: Sink<O> + 'static,
    {
        CompleteTopology {
            source: self.source,
            steps: self.steps,
            sink,
        }
    }
}

struct TransformStep<T, I, O> {
    transform: T,
}

impl<T, I, O> Step<I> for TransformStep<T, I, O>
where
    T: Transform<I, O> + 'static,
    I: Send + 'static,
    O: Send + 'static,
{
    fn run(self: Box<Self>, input: Receiver<Event<I>>) -> BoxFuture<'static, ()> {
        let (tx, rx) = channel(16);
        let fut = async move {
            self.transform.run(input, tx).await;
        };
        Box::pin(fut)
    }
}

struct CompleteTopology<S, O, K> {
    source: S,
    steps: Vec<Box<dyn Step<O>>>,
    sink: K,
}

impl<S, O, K> CompleteTopology<S, O, K>
where
    S: Source<O> + Send + 'static,
    K: Sink<O> + Send + 'static,
    O: Send + 'static,
{
    pub async fn start(self) {
        let (tx, rx) = channel(16);
        let source_handle = tokio::spawn(self.source.run(tx));
        let mut prev_rx = rx;

        // Run all steps (transforms)
        for step in self.steps {
            let fut = step.run(prev_rx);
            prev_rx = /* get the output receiver from the step */;
            tokio::spawn(fut);
        }

        // Sink
        let sink_handle = tokio::spawn(self.sink.run(prev_rx));

        let _ = tokio::join!(source_handle, sink_handle);
    }
}