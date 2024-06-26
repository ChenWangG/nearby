cmake_minimum_required(VERSION 3.14...3.28)
project(system_api_android)

find_package(Java REQUIRED)
include(UseJava)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --example test_api)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --example test_api)
    set(TARGET_DIR "release")
endif ()

set(TEST_API_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/examples/libtest_api.so")

message(STATUS "Cargo build Rust TEST API SO.")
add_custom_target(test_api ALL
    COMMENT "Compiling test_api.rs"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${TEST_API_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_jar(Main examples/Main.java)
get_target_property(_MainJarFile Main JAR_FILE)
message(STATUS "Main Jar file ${_jarFile}")

add_custom_target(nearby_android_example ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -cp ${_MainJarFile} Main)
add_dependencies(nearby_android_example Main)