use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::task;
use tokio::task::JoinHandle;

pub trait EventProcessor: Send {
    type Event;

    fn process(&mut self, event: Option<Self::Event>) -> impl std::future::Future<Output = ()> + Send;
}

enum PollerEvent<E> {
    Stop,
    Event(E),
}

pub struct EventWriter<E> {
    sender: mpsc::Sender<PollerEvent<E>>,
}

impl<E> EventWriter<E> {
    pub async fn write(&self, event: E) -> Result<(), SendError<()>> {
        match self.sender.send(PollerEvent::Event(event)).await {
            Ok(_) =>  Ok(()),
            Err(_) => Err(SendError(())),
        }
    }

    pub async fn stop(&self) {
        self.sender.send(PollerEvent::Stop).await;
    }
}

pub struct EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static + Clone {
    processor: P,
    receiver: mpsc::Receiver<PollerEvent<P::Event>>,
}

impl<P> EventPoller<P>
    where P: EventProcessor + Send + 'static,
          P::Event: Send + 'static + Clone {

    pub fn create(processor: P) -> (EventWriter<P::Event>, Self) {
        let (sender, receiver) = mpsc::channel(32);
        (EventWriter{sender}, EventPoller{receiver, processor})
    }

    pub fn processor(&mut self) -> &mut P {
        &mut self.processor
    }

    pub fn start(mut self) -> JoinHandle<()> {
        println!("start poll.");
        task::spawn(async move {
            loop {
                match self.receiver.recv().await {
                    Some(PollerEvent::Stop) => self.receiver.close(),
                    Some(PollerEvent::Event(event)) => self.processor.process(Some(event)).await,
                    None => {
                        self.processor.process(None).await;
                        break;
                    }
                }
            }
        })
    }
}