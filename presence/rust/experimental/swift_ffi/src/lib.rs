pub type CallSwift = fn(i32);

#[no_mangle]
pub extern fn test_swift_ffi() -> i32 {
  println!("Test Swift FFI");
  17
}

#[no_mangle]
pub unsafe extern fn test_swift_callback(call_swift: CallSwift) -> i32 {
  println!("Test Swift FFI");
  call_swift(3);
  18
}
