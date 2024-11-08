cmake_minimum_required(VERSION 3.14...3.28)
project(presence_android)

add_subdirectory(presence_java)

message(STATUS "Copy libpresence_java.so to jinLibs.")
add_custom_target(presence_android ALL
    COMMENT "Copy libpresence_java.so to Android"
    COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/presence_java/build/libpresence_java.so
               ${CMAKE_CURRENT_SOURCE_DIR}/src/main/jniLibs/arm64-v8a/
    COMMAND cp ${CMAKE_CURRENT_SOURCE_DIR}/presence_java/build/Presence.jar
               ${CMAKE_CURRENT_SOURCE_DIR}/libs/
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
add_dependencies(presence_android presence_java)
