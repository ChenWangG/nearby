use scan_provider::ble_scan_provider::BleScanProcessor;
use crate::client::{Client, DiscoveryCallback};
use crate::engine::EngineProcessor;
use crate::util::async_block_on;
use event_poller::EventPoller;
use futures::future;
use scan_provider::ble_scan_provider;

pub mod client;
mod engine;
pub mod util;
#[cfg(feature = "mock")]
pub mod mock;
mod scan_provider;

pub fn create<C>() -> (Client, Runtime<C>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (engine, mut engine_poller) = engine::create();
    let (ble_scan_provider, mut ble_scan_poller) = ble_scan_provider::create();
    engine_poller
        .processor()
        .set_ble_scan_provider(ble_scan_provider);
    ble_scan_poller.processor().set_engine(engine.clone());
    (
        Client::new(engine),
        Runtime::new(engine_poller, ble_scan_poller),
    )
}

pub struct Runtime<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    engine_poller: EventPoller<EngineProcessor<C>>,
    ble_scan_poller: EventPoller<BleScanProcessor>,
}

impl<C> Runtime<C>
where
    C: DiscoveryCallback + Send + 'static,
{
    pub fn new(
        engine_poller: EventPoller<EngineProcessor<C>>,
        ble_scan_poller: EventPoller<BleScanProcessor>,
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
