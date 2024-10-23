use event_poller::EventPoller;
use event_poller::EventProcessor;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

#[derive(PartialEq, Clone, Debug)]
enum EchoEvent {
    Closed,
    Number(i32),
}
struct EchoController {
    echo_sender: mpsc::Sender<EchoEvent>,
}

impl EventProcessor for EchoController {
    type Event = EchoEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        // Echo the event back.
        // If the event is None, echo Closed back.
        let echo_event = event.unwrap_or_else(|| EchoEvent::Closed);
        self.echo_sender.send(echo_event).await.unwrap();
    }
}

#[tokio::test]
async fn test_event_poller() {
    let (echo_sender, mut echo_receiver) = mpsc::channel(100);

    let (echo_writer, echo_poller) = EventPoller::create(EchoController { echo_sender });
    echo_poller.start();

    let n = 1;
    assert_eq!(echo_writer.write(EchoEvent::Number(n)).await, Ok(()));
    assert_eq!(echo_receiver.recv().await.unwrap(), EchoEvent::Number(n));

    echo_writer.stop().await;
    assert_eq!(echo_receiver.recv().await.unwrap(), EchoEvent::Closed);
    assert_eq!(
        echo_writer.write(EchoEvent::Number(2)).await,
        Err(SendError(EchoEvent::Number(2)))
    );
}
