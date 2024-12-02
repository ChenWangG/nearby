cmake_minimum_required(VERSION 3.14...3.28)
project(swift_ffi)

message(STATUS "Cargo build to ${CMAKE_CURRENT_BINARY_DIR}")
message(STATUS "CMake source dir ${CMAKE_CURRENT_SOURCE_DIR}")
set(TARGET aarch64-apple-ios)
add_custom_target(swift_ffi ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}  /Users/chenw/.cargo/bin/cargo build --release --lib --target ${TARGET}
    COMMAND rm -rf "${CMAKE_CURRENT_BINARY_DIR}/SwiftFFI.xcframework"
    COMMAND xcodebuild -create-xcframework
        -library "${CMAKE_CURRENT_BINARY_DIR}/${TARGET}/release/libswift_ffi.a"
        -headers ./include/
        -output "${CMAKE_CURRENT_BINARY_DIR}/SwiftFFI.xcframework"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
