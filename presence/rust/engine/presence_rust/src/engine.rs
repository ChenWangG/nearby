use event_poller::EventProcessor;


#[derive(Clone)]
pub enum EngineEvent{
    Ble,
}
pub struct Engine;

impl EventProcessor for Engine {
    type Event = EngineEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
    }
}

impl Engine {
    pub fn new() -> Self {
        Self
    }
}