use std::ffi::c_void;
use std::ptr::null_mut;
use presence_rust::client::{Client, DiscoveryCallback, DiscoveryRequest, DiscoveryResult};
use ble::{Scanner, SwiftStartBleScan};
use presence_rust::Runtime;

use log::{debug, info};
use log::LevelFilter;
use oslog::OsLogger;

use ble::BleScanner;

pub type SwiftOnDiscover = fn(* mut c_void, u32);
struct Callback {
    ios_presence: *mut c_void,
    swift_on_discover: SwiftOnDiscover,
}
// Safe to move pointers.
unsafe impl Send for Callback {}
impl DiscoveryCallback for Callback {
    fn on_update(&self, result: DiscoveryResult) {
        info!("(PresenceRust) DiscoveryCallback on_update");
        info!("(PresenceRust) {}", result.data_elements().len());
        for de in result.data_elements() {
            info!("(PresenceRust) {:?}", de);
            (self.swift_on_discover)(self.ios_presence, de.de_type)
        }
    }
}

// Placeholder for both client and runtime to be "owned" by the host in Swift.
struct PresenceRust {
    ios_presence: *mut c_void,
    client: *mut Client,
    runtime: *mut Runtime<Callback, BleScanner>,
}


#[no_mangle]
pub extern "C" fn presence_new(ios_presence: *mut std::ffi::c_void,
                               swift_start_ble_scan: SwiftStartBleScan) -> *mut PresenceRust  {
    // TODO toggle on metadata in Xcode log view to show the log source.
    OsLogger::new("com.google.test_ble_scanner")
        .level_filter(LevelFilter::Debug)
        .init()
        .unwrap();
    info!("presence_new.");

    let ble_scanner = BleScanner::new(ios_presence, swift_start_ble_scan);
    let (client, mut runtime) = presence_rust::create(ble_scanner);

    let presence_rust = PresenceRust{
        ios_presence,
        client: Box::into_raw(Box::new(client)),
        runtime: Box::into_raw(Box::new(runtime))};
    Box::into_raw(Box::new(presence_rust))
}

#[no_mangle]
pub unsafe extern "C" fn presence_start(presence_rust_ptr: *mut PresenceRust,
                                        swift_on_discover: SwiftOnDiscover) {
    info!("presence_start.");
    let callback = Callback{ ios_presence: (*presence_rust_ptr).ios_presence,  swift_on_discover };

    let mut runtime = Box::from_raw((*presence_rust_ptr).runtime);
    (*presence_rust_ptr).runtime = null_mut();

    runtime.set_discovery_callback(callback);
    runtime.start();
}

#[no_mangle]
pub unsafe extern "C" fn presence_set_request(presence_rust_ptr: *mut PresenceRust) {
    info!("presence_set_request.");
    (*(*presence_rust_ptr).client).set_request(DiscoveryRequest{ priority: 100 });
}


