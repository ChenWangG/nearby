use event_poller::{EventPoller, EventProcessor, EventWriter};
use crate::ble_scan_provider::BleScanProvider;
use crate::client::{DiscoveryCallback, DiscoveryRequest, DiscoveryResult};

pub fn create<C>(callback: C) -> (Engine, EventPoller<EngineProcessor<C>>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (writer, engine_poller) = event_poller::create(EngineProcessor::new(callback));
    (Engine { writer }, engine_poller)
}

#[derive(Clone)]
pub struct Engine {
    writer: EventWriter<EngineEvent>,
}

impl Engine {
    pub async fn set_request(&mut self, request: DiscoveryRequest) {
        self.writer.write(EngineEvent::Ble).await.unwrap();
    }

    pub async fn on_result(&mut self, request: DiscoveryResult) {
        self.writer.write(crate::engine::EngineEvent::Result).await.unwrap();
    }
    pub async fn stop(&mut self) {
        self.writer.stop().await.unwrap();
    }


}

#[derive(Clone, Debug)]
pub enum EngineEvent {
    Ble,
    Mdns,
    Result,
}
pub struct EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    discovery_callback: C,
    ble_scan_provider: Option<BleScanProvider>,
}

impl<C> EventProcessor for EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    type Event = EngineEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        println!("Engine Processor process event starts.");
        match event {
            None => {
                print!("Engine stops ble scan provider.");
                self.ble_scan_provider.as_mut().unwrap().stop().await;
            }
            Some(EngineEvent::Ble) => {
                print!("Engine set ble scan request.");
                self.ble_scan_provider.as_mut().unwrap().set_scan_request().await;
            }
            Some(EngineEvent::Result) => {
                print!("Engine receives discovery result.");
                self.discovery_callback.on_update(DiscoveryResult{});
            }
            _ => { print!("Other event");}
        }
        println!("Engine Processor process event ends.");
    }
}

impl<C> EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    pub fn new(discovery_callback: C) -> Self {
        Self { discovery_callback, ble_scan_provider: None, }
    }

    pub fn set_ble_scan_provider(&mut self, provider: BleScanProvider) {
        self.ble_scan_provider = Some(provider);
    }
}
