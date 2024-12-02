use oslog::OsLogger;
use log::{debug, error, info};
use log::LevelFilter;

pub type CallSwift = fn(i32);

pub struct RustObject {
    call_swift: CallSwift,
}

#[no_mangle]
pub extern fn rust_object_new(call_swift: CallSwift) -> *mut RustObject {
  println!("[Rust] new Rust Object");
  // The log level is enabled in iOS by set env variable
  // RUST_LOG=debug
  // in XCode Product -> Scheme -> Edit Scheme
  // env_logger::init();
  // TODO toggle on metadata in Xcode log view to show the log source.
  OsLogger::new("com.google.SwiftFFI")
      .level_filter(LevelFilter::Debug)
      .init()
      .unwrap();
  info!("new Rust Object.");
  Box::into_raw(Box::new(RustObject { call_swift }))
}

#[no_mangle]
pub unsafe extern "C" fn rust_object_call_swift(rust_object: *mut RustObject) {
  debug!("Rust object call swift.");
  ((*rust_object).call_swift)(3);
}

#[no_mangle]
pub unsafe extern fn test_swift_callback(call_swift: CallSwift) -> i32 {
  println!("Test Swift FFI");
  call_swift(3);
  18
}
