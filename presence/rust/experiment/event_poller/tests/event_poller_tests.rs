use tokio::sync::mpsc;
use event_poller::EventProcessor;
use event_poller::EventPoller;

struct ScanController {
    echo_sender: mpsc::Sender<i32>,
}

impl EventProcessor for ScanController {
    type Event = i32;

    async fn process(&mut self, event: Self::Event) {
        // Echo the event back.
        self.echo_sender.send(event).await.expect("Echo send failed.");
    }
}

#[tokio::test]
async fn test_event_poller() {
    let(echo_sender, mut echo_receiver) = mpsc::channel(32);

    let (scan_controller_writer, scan_controller_poller) =
        EventPoller::create(ScanController{ echo_sender });
    scan_controller_poller.start();

    scan_controller_writer.write(1).await;
    let received_number = echo_receiver.recv().await.unwrap();
    assert_eq!(received_number, 1);
}