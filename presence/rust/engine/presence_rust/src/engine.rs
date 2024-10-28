use event_poller::{EventPoller, EventProcessor, EventWriter};
use crate::ble_scan_provider::BleScanProvider;
use crate::client::{DiscoveryCallback, DiscoveryRequest};

pub fn create<C>(callback: C) -> (Engine, EventPoller<EngineProcessor<C>>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (writer, engine_poller) = event_poller::create(EngineProcessor::new(callback));
    (Engine { writer }, engine_poller)
}

pub struct Engine {
    writer: EventWriter<EngineEvent>,
}

impl Engine {
    pub async fn set_request(&mut self, request: DiscoveryRequest) {
        self.writer.write(EngineEvent::Ble).await.unwrap();
    }

    pub async fn stop(&mut self) {
        self.writer.stop().await.unwrap();
    }
}

#[derive(Clone)]
pub enum EngineEvent {
    Ble,
    Number(i32),
}
pub struct EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    discovery_callback: C,
    ble_scan_provider: Option<BleScanProvider>,
    sum: i32,
}

impl<C> EventProcessor for EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    type Event = EngineEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        match event {
            None => {}
            Some(EngineEvent::Ble) => {}
            Some(EngineEvent::Number(number)) => { self.sum += number; }
        }
    }
}

impl<C> EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    pub fn new(discovery_callback: C) -> Self {
        Self { discovery_callback, ble_scan_provider: None, sum: 0 }
    }

    pub fn set_ble_scan_provider(&mut self, provider: BleScanProvider) {
        self.ble_scan_provider = Some(provider);
    }
}
