cmake_minimum_required(VERSION 3.14...3.28)
project(nearby_ble)

if (ANDROID_HOME_DIR)
    message(STATUS "Cargo command set Android: ${ANDROID_HOME_DIR}")
    set(TARGET aarch64-linux-android)
    set(CARGO_CMD ANDROID_HOME=${ANDROID_HOME_DIR} cargo ndk --platform 30 --target aarch64-linux-android build --examples)
    set(TEST_BLE_SCANNER_SO "${CMAKE_CURRENT_BINARY_DIR}/aarch64-linux-android/debug/examples/libtest_ble_scanner.so")
else ()
    message(STATUS "Cargo command set Linux.")
    set(CARGO_CMD cargo build --examples)
    set(TEST_BLE_SCANNER_SO "${CMAKE_CURRENT_BINARY_DIR}/debug/examples/libtest_ble_scanner.so")
endif ()


message(STATUS "Cargo build ${TEST_BLE_SCANNER_SO}")
add_custom_target(cargo_build_test_ble_scan ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${TEST_BLE_SCANNER_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/..)

if (ANDROID_HOME_DIR)
    return ()
endif ()

find_package(Java REQUIRED)
include(UseJava)
add_jar(TestBle
        "unit_test/Main.java"
        "unit_test/BleWrapper.java"
        "ble_test/TestBleScanner.java"
         "ble/BleScanner.java"
         "ble/ScanResult.java")
get_target_property(_testBleJarFile TestBle JAR_FILE)
add_dependencies(TestBle cargo_build_test_ble_scan)
message(STATUS "Created Jar file ${_testBleJarFile}")

message(STATUS "Run BLE Java tests.")
enable_testing()
add_test(NAME test_ble_java
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_testBleJarFile} -ea Main)
set_tests_properties(test_ble_java PROPERTIES DEPENDS TestBle)