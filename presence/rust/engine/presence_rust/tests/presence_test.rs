use std::sync::mpsc;
use presence_rust::client::{Client, DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use presence_rust::util;
use std::thread;

struct Callback {
    tx: mpsc::Sender<DiscoveryResult>,
}

impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        print!("DiscoveryCallback on_update");
        self.tx.send(result).unwrap();
    }
}

#[test]
fn test_engine() {
    thread::scope(|scope| {
        let (tx, rx) = mpsc::channel();
        let callback = Callback { tx };
        let (mut client, runtime) = presence_rust::create(callback);
        let runtime_thread = scope.spawn(|| runtime.start());
        client.set_request(DiscoveryRequest{});
        rx.recv().unwrap();
        client.stop();
        runtime_thread.join().expect("Presence test crashed.");
    });
}
