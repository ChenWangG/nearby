use presence_rust::client::{Client, DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use ble::{Scanner, SwiftStartBleScan};
use presence_rust::Runtime;

use log::{debug, info};
use log::LevelFilter;
use oslog::OsLogger;

use ble::BleScanner;

struct Callback;
impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        info!("(PresenceRust) DiscoveryCallback on_update");
        info!("(PresenceRust) {}", result.data_elements().len());
        for de in result.data_elements() {
            info!("(PresenceRust) {:?}", de)
        }
    }
}

// Placeholder for both client and runtime to be "owned" by the host in Swift.
struct PresenceRust {
    client: *mut Client,
    runtime: *mut Runtime<Callback, BleScanner>,
}

#[no_mangle]
pub extern "C" fn presence_new(ios_presence: *mut std::ffi::c_void, swift_start_ble_scan: SwiftStartBleScan) -> *mut PresenceRust  {
    // TODO toggle on metadata in Xcode log view to show the log source.
    OsLogger::new("com.google.test_ble_scanner")
        .level_filter(LevelFilter::Debug)
        .init()
        .unwrap();
    info!("presence_new.");

    let ble_scanner = BleScanner::new(ios_presence, swift_start_ble_scan);
    let (client, runtime) = presence_rust::create(ble_scanner);
    let presence_rust = PresenceRust{
        client: Box::into_raw(Box::new(client)),
        runtime: Box::into_raw(Box::new(runtime))};
    Box::into_raw(Box::new(presence_rust))
}
