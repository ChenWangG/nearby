#[no_mangle]
pub extern fn test_swift_ffi() -> i32 {
  println!("Test Swift FFI");
  17
}
