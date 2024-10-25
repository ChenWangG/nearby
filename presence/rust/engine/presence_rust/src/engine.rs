use crate::DiscoveryCallback;
use event_poller::EventProcessor;

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
