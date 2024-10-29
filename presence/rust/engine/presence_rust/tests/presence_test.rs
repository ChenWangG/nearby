use presence_rust::client::{DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
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
        let (mut client, runtime) = presence_rust::create(callback);
        let runtime_thread = scope.spawn(|| runtime.start());
        client.set_request(DiscoveryRequest{});
        client.stop();
        runtime_thread.join().expect("Presence test crashed.");
    });
}
