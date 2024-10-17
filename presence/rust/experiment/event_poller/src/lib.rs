use tokio::sync::mpsc;
use tokio::task;

trait EventProcessor {
    type Event;
    fn process(&mut self, event: Self::Event);
}

struct EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static {
    receiver: mpsc::Receiver<P::Event>,
    processor: P,
}

impl<P> EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static {
    fn start(mut self) -> task::JoinHandle<()> {
        println!("start poll.");
        task::spawn(async move {
            while let Some(event) = self.receiver.recv().await {
                self.processor.process(event);
            }
        })
    }
}