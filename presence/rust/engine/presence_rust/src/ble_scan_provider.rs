use crate::engine::Engine;
use event_poller::{EventPoller, EventProcessor, EventWriter};
use std::future::Future;
use ble::{BleScanRequest, BleScanResult, ScanCallback, Scanner};
#[cfg(not(feature = "mock"))]
use ble::BleScanner;
#[cfg(feature = "mock")]
use crate::mock_ble::BleScanner;
#[cfg(feature = "mock")]
use crate::mock_ble::TestBleScanner;

pub fn create() -> (BleScanProvider, EventPoller<BleScanProcessor>) {
    let _ = TestBleScanner;

    let ble_scanner = BleScanner{};
    let (writer, ble_scan_poller) = event_poller::create(BleScanProcessor { engine: None, ble_scanner });
    (BleScanProvider { writer }, ble_scan_poller)
}

#[derive(Clone)]
pub struct BleScanProvider {
    writer: EventWriter<BleScanEvent>,
}

impl BleScanProvider {
    pub async fn set_scan_request(&self) {
        self.writer.write(BleScanEvent::Start).await;
    }
}

pub(crate) struct BleScanProcessor {
    engine: Option<Engine>,
    ble_scanner: BleScanner,
}

impl EventProcessor for BleScanProcessor {
    type Event = BleScanEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        match event {
            None => {}
            Some(BleScanEvent::Start) => {
                println!("Received BleScanEvent::Start.");
                self.ble_scanner.start(BleScanRequest{}, BleScanCallback{});
            }
            _ => panic!("Recived None BleScanEvent::Start."),
        }
    }
}

impl BleScanProcessor {
    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }
}

#[derive(Clone)]
pub enum BleScanEvent {
    Start,
    Stop,
}

struct BleScanCallback;

impl ScanCallback for BleScanCallback {
    fn on_update(&self, result: BleScanResult) {
        todo!()
    }
}
