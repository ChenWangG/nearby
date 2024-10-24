use event_poller::{EventPoller, EventWriter};
use crate::engine::{Engine, EngineEvent};
use tokio::runtime::Builder;
use  std::future::Future;
use std::sync::Mutex;
use log::{debug, error, info};

mod engine;

// Implemented by the client to receive discovery results.
pub trait DiscoveryCallback {
    fn on_update(&self, result: DiscoveryResult);
}

pub struct Presence<C>
where C: DiscoveryCallback {
    discovery_callback: C,
    engine_writer: Option<EventWriter<EngineEvent>>,
    mutex: Mutex<i32>,
}

impl<C> Presence<C>
where C: DiscoveryCallback {
    pub fn new(discovery_callback: C)  -> Self {
        Self { discovery_callback, engine_writer: None, mutex: Mutex::new(0), }
    }

    pub fn set_request(&mut self) {
        let _lock = self.mutex.lock().unwrap();
        if self.engine_writer.is_none() {
            info!("Send a request to the Engine which is not started.");
            return;
        }
        if let Some(writer) = self.engine_writer.clone() {
            Self::async_block_on(async move { writer.write(EngineEvent::Ble).await.unwrap() });
        }
    }

    pub fn start(&mut self) {
        let lock = self.mutex.lock().unwrap();
        if self.engine_writer.is_some() {
            info!("Start the Engine which is already started.");
            return;
        }
        let (engine_writer, mut engine_poller) = EventPoller::create(Engine::new());
        self.engine_writer = Some(engine_writer);
        // unlock the mutex.
        std::mem::drop(lock);
        Self::async_block_on(async move { engine_poller.start().await; });
    }

    pub fn stop(&mut self) {
        let _lock = self.mutex.lock().unwrap();
        if let Some(writer) = self.engine_writer.clone() {
            Self::async_block_on(async move { writer.stop().await.unwrap() });
        }
        self.engine_writer = None;
    }

    fn async_block_on(future: impl Future<Output = ()>) {
        Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(future);
    }
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

impl  PresenceMedium {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
