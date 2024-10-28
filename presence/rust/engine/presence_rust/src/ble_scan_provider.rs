use std::future::Future;
use event_poller::{EventPoller, EventProcessor, EventWriter};

pub fn create() -> (BleScanProvider, EventPoller<BleScanProcessor>)  {
    let (writer, ble_scan_poller) = event_poller::create(BleScanProcessor{});
    (BleScanProvider{writer}, ble_scan_poller)
}

struct BleScanProvider {
    writer: EventWriter<BleScanEvent>,
}

struct BleScanProcessor;

impl EventProcessor for BleScanProcessor {
    type Event = BleScanEvent;

    async fn process(&mut self, event: Option<Self::Event>) {
        todo!()
    }
}

#[derive(Clone)]
enum BleScanEvent {}