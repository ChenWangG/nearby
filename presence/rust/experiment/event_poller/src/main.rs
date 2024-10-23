use futures::executor::block_on;
use async_std::task::spawn;
use tokio::sync::mpsc;
// test
struct EventLoop {
    rx: mpsc::Receiver<i32>,
}

impl EventLoop {
    pub async fn start(&mut self) {
        println!("Start event loop.");
        while let Some(number) = self.rx.recv().await {
            println!("received: {}", number);
        }
    }
    pub async fn stop(&mut self) {
        println!("Stop event loop.");
    }
}

async fn run_event_loop() {
    let (tx, rx) = mpsc::channel(32);
    let mut event_poller = EventLoop {rx};
    tx.send(1).await.unwrap();
    spawn(async move {event_poller.start().await});
    for i in 1..100 {
        tx.send(i).await.unwrap();
    }
    // event_poller.stop().await;
}

trait EventProcessor {
    type Event;
    fn process(&self, event: Self::Event);
}

struct EventPoller<P: EventProcessor> {
    processor: P,
}

impl<P: EventProcessor> EventPoller<P> {
    fn start(self, event: P::Event) {
        println!("start poll.");
        self.processor.process(event);
    }
}

enum ScanControllerEvent {
    BLE,
}

struct ScanController {}

impl EventProcessor for ScanController {
    type Event = ScanControllerEvent;
    fn process(&self, event: ScanControllerEvent) {
        println!("Scan Controller process.")
    }
}

impl EventPoller<ScanController> {
    fn add_ble_scanner(&self) {
        println!("Add BLE scanner.");
    }
}


fn main() {
    println!("Hello, world!");
    /*
    let scan_controller = ScanController{};
    scan_controller.process(ScanControllerEvent::BLE);
    let poller = EventPoller{ processor: ScanController{} };
    poller.add_ble_scanner();
    poller.start(ScanControllerEvent::BLE);

     */
    block_on(run_event_loop());
}
