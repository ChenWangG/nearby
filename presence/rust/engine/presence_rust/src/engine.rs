use crate::{async_block_on, DiscoveryCallback, DiscoveryResult};
use event_poller::{EventPoller, EventProcessor, EventWriter};

pub fn create<C>(callback: C) -> (Engine, EventPoller<EngineProcessor<C>>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (engine_writer, engine_poller) = event_poller::create(EngineProcessor::new(callback));
    (Engine::new(engine_writer), engine_poller)

}

pub struct Engine {
    writer: EventWriter<EngineEvent>,
}

impl Engine {
    pub fn new(writer: EventWriter<EngineEvent>) -> Self {
        Self{ writer }
    }
    pub async fn set_request(&mut self, request: DiscoveryResult) {
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
