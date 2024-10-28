use presence_rust::client::{DiscoveryCallback, DiscoveryResult};
use presence_rust::util;
use std::thread;

struct Callback;

impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        todo!()
    }
}
#[test]
fn test_engine() {
    thread::scope(|scope| {
        let callback = Callback {};
        let (mut client, engine_poller) = presence_rust::client::create(callback);
        let engine_thread = scope.spawn(|| {
            util::async_block_on(async move {
                // blocked by the returned handle.
                engine_poller.start().await.unwrap()
            })
        });
        client.stop();
        engine_thread.join().expect("Presence test crashed.");
    });
}
