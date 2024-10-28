use crate::ble_scan_provider::BleScanProcessor;
use crate::client::{Client, DiscoveryCallback};
use crate::engine::EngineProcessor;
use crate::util::async_block_on;
use event_poller::EventPoller;

mod ble_scan_provider;
pub mod client;
mod engine;
pub mod util;

pub fn create<C>(callback: C) -> (Client, Runtime<C>)
where
    C: DiscoveryCallback + Send + 'static,
{
    let (engine, mut engine_poller) = engine::create(callback);
    let (ble_scan_provider, ble_scan_poller) = ble_scan_provider::create();
    engine_poller.processor().set_ble_scan_provider(ble_scan_provider);
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

    pub fn start(self) {
        async_block_on(async move {
            // TODO: blocked by all handles.
            // blocked by the returned handle.
            self.ble_scan_poller.start();
            self.engine_poller.start().await.unwrap()
        });
    }
}
