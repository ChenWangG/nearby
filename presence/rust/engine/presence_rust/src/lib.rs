use crate::engine::{EngineProcessor, EngineEvent, Engine};
use event_poller::{EventPoller, EventWriter};
use std::future::Future;
use std::marker::Send;
use tokio::runtime::Builder;

mod engine;

// Implemented by the client to receive discovery results.
pub trait DiscoveryCallback {
    fn on_update(&self, result: DiscoveryResult);
}

pub fn create<C>(callback: C) -> (Client, Runtime<C>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (engine, engine_poller) = engine::create(callback);
    (Client::new(engine), Runtime::new(engine_poller))
}

pub struct Client {
    engine: Engine,
}

impl Client {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn set_request(&mut self, request: DiscoveryRequest) {
        async_block_on(async move { self.engine.set_request(request).await; });
    }

    pub fn stop(&mut self) {
        async_block_on(async move { self.engine.stop().await; });
    }
}
pub struct Runtime<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    event_poller: EventPoller<EngineProcessor<C>>,
}

impl<C> Runtime<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    pub fn new(event_poller: EventPoller<EngineProcessor<C>>) -> Self {
        Self { event_poller }
    }

    pub fn start(self) {
        async_block_on(async move { self.event_poller.start().await.unwrap() });
    }
}

fn async_block_on(future: impl Future<Output = ()>) {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(future);
}

// The enum is annotated by repr(C) to pass through FFI.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum PresenceMedium {
    Unknown = 0,
    BLE,
    WiFiRTT,
    UWB,
    MDNS,
}

impl PresenceMedium {
    pub fn from_i32(value: i32) -> PresenceMedium {
        match value {
            0 => PresenceMedium::Unknown,
            1 => PresenceMedium::BLE,
            2 => PresenceMedium::WiFiRTT,
            3 => PresenceMedium::UWB,
            4 => PresenceMedium::MDNS,
            _ => panic!("Unknown PresenceMedium value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub actions: Vec<i32>,
}

impl Device {
    pub fn new(actions: Vec<i32>) -> Self {
        Self { actions }
    }
}
#[derive(Debug)]
pub struct DiscoveryResult {
    pub medium: PresenceMedium,
    pub device: Device,
}

impl DiscoveryResult {
    pub fn new(medium: PresenceMedium, device: Device) -> Self {
        Self { medium, device }
    }
}

pub struct DiscoveryRequest;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
