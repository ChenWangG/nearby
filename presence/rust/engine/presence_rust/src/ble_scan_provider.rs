use event_poller::{EventPoller, EventProcessor, EventWriter};
use std::future::Future;

pub fn create() -> (BleScanProvider, EventPoller<BleScanProcessor>) {
    let (writer, ble_scan_poller) = event_poller::create(BleScanProcessor {});
    (BleScanProvider { writer }, ble_scan_poller)
}

pub struct BleScanProvider {
    writer: EventWriter<BleScanEvent>,
}

pub(crate) struct BleScanProcessor;

impl EventProcessor for BleScanProcessor {
    type Event = BleScanEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        todo!()
    }
}

#[derive(Clone)]
pub enum BleScanEvent {}
