use async_std::task::spawn;
use futures::executor::block_on;
use futures::future::join_all;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task;
use event_poller::EventProcessor;
use event_poller::EventPoller;

struct ScanController {
    sender: Sender<i32>,
}

impl EventProcessor for ScanController {
    type Event = i32;

    async fn process(&mut self, event: Self::Event) {
        // Echo the event back.
        self.sender.send(event).await.unwrap();
    }
}

#[tokio::test]
async fn test_event_poller() {
    let(sender, mut receiver) = mpsc::channel(32);
    let (scan_controller_writer, scan_controller_poller) =
        EventPoller::create(ScanController{sender});
    scan_controller_poller.start();
    scan_controller_writer.write(1).await;
    let received_number = receiver.recv().await.unwrap();
    assert_eq!(received_number, 1);
}