use scan_provider::ble_scan_provider::BleScanProcessor;
use crate::client::{Client, DiscoveryCallback};
use crate::engine::EngineProcessor;
use crate::util::async_block_on;
use event_poller::EventPoller;
use futures::future;
use ble::Scanner;
use scan_provider::ble_scan_provider;

pub mod client;
mod engine;
pub mod util;
pub mod mock;
mod scan_provider;

pub fn create<C: DiscoveryCallback, S: Scanner>(ble_scanner: S) -> (Client, Runtime<C, S>) {
    let (engine, mut engine_poller) = engine::create();
    let (ble_scan_provider, mut ble_scan_poller) = ble_scan_provider::create(ble_scanner);
    engine_poller
        .processor()
        .set_ble_scan_provider(ble_scan_provider);
    ble_scan_poller.processor().set_engine(engine.clone());
    (
        Client::new(engine),
        Runtime::new(engine_poller, ble_scan_poller),
    )
}

pub struct Runtime<C: DiscoveryCallback, S: Scanner> {
    engine_poller: EventPoller<EngineProcessor<C>>,
    ble_scan_poller: EventPoller<BleScanProcessor<S>>,
}

impl<C: DiscoveryCallback, S: Scanner> Runtime<C, S> {
    pub fn new(
        engine_poller: EventPoller<EngineProcessor<C>>,
        ble_scan_poller: EventPoller<BleScanProcessor<S>>,
    ) -> Self {
        Self {
            engine_poller,
            ble_scan_poller,
        }
    }

    pub fn set_discovery_callback(&mut self, callback: C) {
        self.engine_poller.processor().set_discovery_callback(callback);
    }

    pub fn start(self) {
        println!("Runtime start.");
        async_block_on(async move {
            let results = future::join_all(vec![
                self.ble_scan_poller.start(),
                self.engine_poller.start(),
            ]).await;
            for result in results {
                result.unwrap().unwrap();
            }
        });
    }
}
