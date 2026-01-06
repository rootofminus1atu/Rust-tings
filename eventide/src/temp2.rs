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

trait Transform<I, O> {
    async fn process(&mut self, input: Event<I>) -> Event<O>;

    async fn run(&mut self, mut receiver: Receiver<Event<I>>, sender: Sender<Event<O>>) {
        while let Some(e) = receiver.recv().await {
            let e = self.process(e).await;
            sender.send(e).await;
        }
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