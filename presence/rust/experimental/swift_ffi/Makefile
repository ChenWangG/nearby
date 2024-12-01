ios:
	@cargo build --release --lib --target aarch64-apple-ios
	@$(RM) -rf libs/libswift_ffi_ios.a
	@cp target/aarch64-apple-ios/release/libswift_ffi.a libs/libswift_ffi_ios.a
