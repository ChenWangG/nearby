use tokio::sync::mpsc;
use tokio::task;


pub trait EventProcessor: Send {
    type Event;

    fn process(&mut self, event: Self::Event) -> impl std::future::Future<Output = ()> + Send;
}

enum PollerEvent<E> {
    Stop,
    Event(E),
}

pub struct EventWriter<E> {
    sender: mpsc::Sender<PollerEvent<E>>,
}

impl<E> EventWriter<E> {
    pub async fn write(&self, event: E) {
        if let Ok(permit) = self.sender.reserve().await {
            permit.send(PollerEvent::Event(event));
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

    pub fn start(mut self) {
        println!("start poll.");
        task::spawn(async move {
            while let Some(poller_event) = self.receiver.recv().await {
                match poller_event {
                    PollerEvent::Stop => {
                        self.receiver.close();
                        break;
                    }
                    PollerEvent::Event(event) => self.processor.process(event).await,
                }
            }
        });
    }
}