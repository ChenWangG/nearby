use tokio::sync::mpsc;
use tokio::task;

pub trait EventProcessor {
    type Event;

    fn new() -> Self;
    fn process(&mut self, event: Self::Event);
}
pub struct EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static + Clone {
    processor: P,
    receiver: mpsc::Receiver<P::Event>,
}

impl<P> EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static + Clone {

    pub fn create(processor: P) -> (mpsc::Sender<P::Event>, Self) {
        let (sender, receiver) = mpsc::channel(32);
        (sender, EventPoller{receiver, processor})
    }

    pub fn processor(&mut self) -> &mut P {
        &mut self.processor
    }

    pub fn start(mut self) -> task::JoinHandle<()> {
        println!("start poll.");
        task::spawn(async move {
            while let Some(event) = self.receiver.recv().await {
                self.processor.process(event);
            }
        })
    }
}