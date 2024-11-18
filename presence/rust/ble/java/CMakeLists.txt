find_package(Java REQUIRED)
include(UseJava)

add_jar(TestBle
        "unit_test/Main.java"
        "unit_test/BleWrapper.java"
        "ble_test/TestBleScanner.java"
        "ble/BleWrapperInterface.java"
        "ble/BleScanner.java"
        "ble/ScanResult.java")
get_target_property(_testBleJarFile TestBle JAR_FILE)
add_dependencies(TestBle nearby_ble)
message(STATUS "Created Jar file ${_testBleJarFile}")

message(STATUS "Run BLE Java tests.")
add_test(NAME test_ble_java
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}/..
        -cp ${_testBleJarFile} -ea Main)
set_tests_properties(test_ble_java PROPERTIES DEPENDS TestBle)