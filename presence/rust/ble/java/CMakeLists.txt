cmake_minimum_required(VERSION 3.14...3.28)
project(nearby_ble)

find_package(Java REQUIRED)
include(UseJava)


# For tests.
enable_testing()

set(TEST_BLE_SCANNER_SO "${CMAKE_CURRENT_BINARY_DIR}/debug/examples/libtest_ble_scanner.so")
message(STATUS "Cargo build ${TEST_BLE_SCANNER_SO}")
add_custom_target(cargo_build_test_ble_scan ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo test
    COMMAND cp ${TEST_BLE_SCANNER_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_jar(TestBle "unit_test/Main.java" "ble_test/TestBleScanner.java" "ble/BleScanner.java")
get_target_property(_testBleJarFile TestBle JAR_FILE)
add_dependencies(TestBle cargo_build_test_ble_scan)
message(STATUS "Created Jar file ${_testBleJarFile}")

message(STATUS "Run BLE Java tests.")
add_test(NAME test_ble_java
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_testBleJarFile} -ea Main)
set_tests_properties(test_ble_java PROPERTIES DEPENDS TestBle)