use event_poller::{EventPoller, EventProcessor, EventWriter};
use crate::scan_provider::ble_scan_provider::BleScanProvider;
use crate::client::{DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use crate::scan_provider::{ScanProvider, ScanRequest, ScanResult};

pub fn create<C: DiscoveryCallback>() -> (Engine, EventPoller<EngineProcessor<C>>) {
    let (writer, engine_poller) = event_poller::create(EngineProcessor::new());
    (Engine { writer }, engine_poller)
}

#[derive(Clone)]
pub struct Engine {
    writer: EventWriter<EngineEvent>,
}

impl Engine {
    pub async fn set_request(&mut self, request: DiscoveryRequest) {
        assert_eq!(request.priority, 100);
        self.writer.write(EngineEvent::DiscoveryRequest(request)).await.unwrap();
    }

    pub async fn on_scan_result(&mut self, result: ScanResult) {
        self.writer.write(crate::engine::EngineEvent::ScanResult(result)).await.unwrap();
    }
    pub async fn stop(&mut self) {
        self.writer.stop().await.unwrap();
    }


}

#[derive(Clone, Debug)]
pub enum EngineEvent {
    Ble,
    Mdns,
    DiscoveryRequest(DiscoveryRequest),
    ScanResult(ScanResult),
}
pub struct EngineProcessor<C: DiscoveryCallback> {
    discovery_callback: Option<C>,
    ble_scan_provider: Option<BleScanProvider>,
}

impl<C> EventProcessor for EngineProcessor<C>
where
    C: DiscoveryCallback,
{
    type Event = EngineEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        println!("Engine Processor process event starts.");
        match event {
            None => {
                print!("Engine stops ble scan provider.");
                self.ble_scan_provider.as_mut().unwrap().stop().await;
            }
            Some(EngineEvent::DiscoveryRequest(request)) => {
                print!("Engine set ble scan request.");
                self.ble_scan_provider.as_mut().unwrap().set_request(ScanRequest{ priority: request.priority}).await;
            }
            Some(EngineEvent::ScanResult(scan_result)) => {
                print!("Engine receives discovery result.");
                self.discovery_callback.as_mut().unwrap().on_update(DiscoveryResult::new(scan_result.service_data().clone()));
            }
            _ => { print!("Other event");}
        }
        println!("Engine Processor process event ends.");
    }
}

impl<C: DiscoveryCallback> EngineProcessor<C> {
    pub fn new() -> Self {
        Self { discovery_callback: None, ble_scan_provider: None, }
    }

    pub fn set_ble_scan_provider(&mut self, provider: BleScanProvider) {
        self.ble_scan_provider = Some(provider);
    }
    pub fn set_discovery_callback(&mut self, callback: C) {
        self.discovery_callback = Some(callback);
    }
}