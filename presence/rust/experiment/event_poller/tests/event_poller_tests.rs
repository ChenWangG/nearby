use async_std::task;
use futures::executor::block_on;
use futures::future::join_all;
use tokio::sync::mpsc;
use event_poller::EventProcessor;
use event_poller::EventPoller;
#[derive(Clone)]
enum ScanEvent {
    BLE,
}

#[derive(Clone)]
enum ScanProviderEvent {}
struct ScanController {
    ble_scan_sender: Option<mpsc::Sender<ScanProviderEvent>>,
}
impl EventProcessor for ScanController {
    type Event = ScanEvent;

    fn new() -> Self {
        Self { ble_scan_sender: None}
    }
    fn process(&mut self, event: ScanEvent) {
        println!("Scan Controller process.")
    }
}

impl ScanController {
    fn add_ble_scan_provider(&mut self, writer: mpsc::Sender<ScanProviderEvent>) {
       self.ble_scan_sender = Some(writer);
    }
}

struct BleScanProvider {
    scan_controller_sender: Option<mpsc::Sender<ScanEvent>>,
}
impl EventProcessor for crate::BleScanProvider {
    type Event = ScanProviderEvent;

    fn new() -> Self {
        Self {scan_controller_sender: None}
    }
    fn process(&mut self, event: ScanProviderEvent) {
        println!("Scan Provider process.")
    }
}

impl BleScanProvider {
    fn add_scan_controller(&mut self, writer: mpsc::Sender<ScanEvent>) {
        self.scan_controller_sender = Some(writer);
    }
}

fn start() -> async_std::task::JoinHandle<()> {
    task::spawn(async move { assert_eq!(1, 1); })
}
async fn run() {
    // task::spawn(async move { assert_eq!(1, 1); }).await;
    assert_eq!(1, 1);
    start().await
}

#[tokio::test]
async fn test_run() {
    run().await;
}

#[tokio::test]
async fn test_event_poller() {
    let (scan_controller_sender, mut scan_controller_poller) =
        EventPoller::create(ScanController::new());

    let (ble_scan_provider_sender, mut ble_scan_provider_poller) =
        EventPoller::create(BleScanProvider::new());

    scan_controller_poller.processor().add_ble_scan_provider(ble_scan_provider_sender);
    ble_scan_provider_poller.processor().add_scan_controller(scan_controller_sender);

    let tasks = vec![scan_controller_poller.start(), ble_scan_provider_poller.start()];

    // join_all(tasks).await;
}