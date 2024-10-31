use crate::engine::Engine;
use crate::util::async_block_on;
use event_poller::EventProcessor;
use std::future::Future;

// Implemented by the client to receive discovery results.
pub trait DiscoveryCallback {
    fn on_update(&self, result: DiscoveryResult);
}

#[derive(Clone)]
pub struct Client {
    engine: Engine,
}

impl Client {
    pub fn new(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn set_request(&mut self, request: DiscoveryRequest) {
        async_block_on(async move {
            self.engine.set_request(request).await;
        });
    }

    pub fn stop(&mut self) {
        async_block_on(async move {
            self.engine.stop().await;
        });
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
pub struct DiscoveryResult{
    service_data: Vec<u8>,
}

impl DiscoveryResult {
    pub fn new(service_data: Vec<u8>) -> Self {
        Self { service_data }
    }

    pub fn service_data(&self) -> &Vec<u8> {
       &self.service_data
    }
}

#[derive(Clone, Debug)]
pub struct DiscoveryRequest {
    pub priority: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
