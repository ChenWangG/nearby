use crate::engine::Engine;
#[cfg(feature = "mock")]
use crate::mock::ble::BleScanner;
#[cfg(not(feature = "mock"))]
use ble::BleScanner;
use ble::{BleScanRequest, BleScanResult, ScanCallback, Scanner};
use event_poller::{EventPoller, EventProcessor, EventWriter};
use std::future::Future;
use crate::ble_scan_provider;

pub const UUID: &str = "0000";

pub fn create() -> (BleScanProvider, EventPoller<BleScanProcessor>) {
    let ble_scanner = BleScanner {};
    let (writer, mut ble_scan_poller) = event_poller::create(BleScanProcessor {
        engine: None,
        ble_scan_provider: None,
        ble_scanner,
    });
    let ble_scan_provider = BleScanProvider{writer};
    ble_scan_poller.processor().set_provider(ble_scan_provider.clone());
    (ble_scan_provider, ble_scan_poller)
}

#[derive(Clone)]
pub struct BleScanProvider {
    writer: EventWriter<BleScanEvent>,
}

impl BleScanProvider {
    pub async fn set_scan_request(&self) {
        self.writer.write(BleScanEvent::Start).await;
    }

    pub async fn stop(&mut self) {
        self.writer.stop().await.unwrap();
    }
}

pub(crate) struct BleScanProcessor {
    engine: Option<Engine>,
    ble_scan_provider: Option<BleScanProvider>,
    ble_scanner: BleScanner,
}

impl EventProcessor for BleScanProcessor {
    type Event = BleScanEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        match event {
            None => {}
            Some(BleScanEvent::Start) => {
                println!("Received BleScanEvent::Start.");
                self.ble_scanner
                    .start(BleScanRequest::new(String::from(UUID)), BleScanCallback { });
            }
            _ => panic!("Recived None BleScanEvent::Start."),
        }
    }
}

impl BleScanProcessor {
    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }
    pub fn set_provider(&mut self, ble_scan_provider: BleScanProvider) {
        self.ble_scan_provider = Some(ble_scan_provider);
    }
}

#[derive(Clone)]
pub enum BleScanEvent {
    Start,
    Stop,
}

struct BleScanCallback {
}

impl ScanCallback for BleScanCallback {
    fn on_update(&self, result: BleScanResult) {

    }
}
