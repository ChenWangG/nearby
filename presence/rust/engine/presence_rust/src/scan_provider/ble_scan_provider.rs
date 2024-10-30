use crate::engine::Engine;
#[cfg(feature = "mock")]
use crate::mock::ble::BleScanner;
#[cfg(not(feature = "mock"))]
use ble::BleScanner;
use ble::{BleScanRequest, BleScanResult, ScanCallback, Scanner};
use event_poller::{EventPoller, EventProcessor, EventWriter};
use crate::client::DiscoveryResult;
use crate::scan_provider::{ScanProvider, ScanRequest, ScanResult};
use crate::util::async_block_on;

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

impl ScanProvider for BleScanProvider {
    async fn set_request(&self, request: ScanRequest) {
        self.writer.write(BleScanEvent::Start).await;
    }
    async fn on_result(&self, result: ScanResult) {
        self.writer.write(BleScanEvent::Result).await;
    }

    async fn stop(&mut self) {
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
                let ble_scan_provider = self.ble_scan_provider.clone().unwrap();
                self.ble_scanner
                    .start(BleScanRequest::new(String::from(UUID)), BleScanCallback { ble_scan_provider });
            }
            Some(BleScanEvent::Result) => {
                println!("Received BleScanEvent::Result.");
                self.engine.as_mut().unwrap().on_result(DiscoveryResult{}).await;
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
    Result,
}

struct BleScanCallback {
    ble_scan_provider: BleScanProvider,
}

impl ScanCallback for BleScanCallback {
    fn on_update(&self, result: BleScanResult) {
        let scan_provider = self.ble_scan_provider.clone();
        async_block_on(async move {
            scan_provider.on_result(ScanResult{ service_data: vec![1, 2, 3]}).await;
        });
    }
}
