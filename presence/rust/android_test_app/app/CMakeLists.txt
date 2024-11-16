cmake_minimum_required(VERSION 3.14...3.28)
project(presence_android)

add_subdirectory(presence_java)
add_subdirectory(ble)

message(STATUS "Copy libpresence_java.so to jinLibs. ${CMAKE_CURRENT_BINARY_DIR}")
add_custom_target(presence_android ALL
    COMMENT "Copy libpresence_java.so to Android"
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/presence_java/libpresence_java.so
               ${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs/arm64-v8a/
    COMMENT "Copy libpresence_java.so to Android"
    COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/ble/libtest_ble_scanner.so
               ${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs/arm64-v8a/
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
add_dependencies(presence_android presence_java nearby_ble)
