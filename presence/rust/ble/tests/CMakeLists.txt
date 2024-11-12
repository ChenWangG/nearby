

add_jar(TestBle TestBle.java)
get_target_property(_testBleJarFile TestBle JAR_FILE)
message(STATUS "Created Jar file ${_testBleJarFile}")

message(STATUS "Run BLE Java tests.")
add_custom_target(ble_java_test ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_testBleJarFile} TestBle)
add_dependencies(ble_java_test TestBle)