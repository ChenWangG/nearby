add_jar(TestBle TestBle.java TestBleScanner.java)
get_target_property(_testBleJarFile TestBle JAR_FILE)
message(STATUS "Created Jar file ${_testBleJarFile}")

message(STATUS "Run BLE Java tests.")
add_test(NAME test_ble_java
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_testBleJarFile} -ea TestBle)
set_tests_properties(test_ble_java PROPERTIES DEPENDS TestBle)

