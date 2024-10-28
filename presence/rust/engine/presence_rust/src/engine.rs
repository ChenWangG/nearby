use crate::{async_block_on, DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use event_poller::{EventPoller, EventProcessor, EventWriter};

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
}
pub struct EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    discovery_callback: C,
}

impl<C> EventProcessor for EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    type Event = EngineEvent;

    async fn process(&mut self, event: Option<Self::Event>) {}
}

impl<C> EngineProcessor<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    pub fn new(discovery_callback: C) -> Self {
        Self { discovery_callback }
    }
}
