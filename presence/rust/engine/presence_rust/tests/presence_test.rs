use std::sync::mpsc;
use presence_rust::client::{DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use std::thread;
use presence_rust::mock::ble::SERVICE_DATA;

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
        client.set_request(DiscoveryRequest{ priority: 100 });
        let result = rx.recv().unwrap();
        assert_eq!(*result.service_data(), SERVICE_DATA);
        client.stop();
        runtime_thread.join().expect("Presence test crashed.");
    });
}
