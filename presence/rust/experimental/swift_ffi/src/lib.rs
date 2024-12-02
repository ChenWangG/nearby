pub type CallSwift = fn(i32);

pub struct RustObject {
    call_swift: CallSwift,
}

#[no_mangle]
pub extern fn rust_object_new(call_swift: CallSwift) -> *mut RustObject {
  println!("[Rust] new Rust Object");
  Box::into_raw(Box::new(RustObject { call_swift }))
}

#[no_mangle]
pub unsafe extern "C" fn rust_object_call_swift(rust_object: *mut RustObject) {
  println!("[Rust] Rust object call swift.");
  ((*rust_object).call_swift)(2);
}

#[no_mangle]
pub unsafe extern fn test_swift_callback(call_swift: CallSwift) -> i32 {
  println!("Test Swift FFI");
  call_swift(3);
  18
}
