cmake_minimum_required(VERSION 3.14...3.28)
project(presence_java)

find_package(Java REQUIRED)
include(UseJava)


if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(PRESENCE_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libpresence_java.so")

message(STATUS "Cargo build Presence Java.")
add_custom_target(presence_java ALL
    COMMENT "Compiling Presence Java module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${PRESENCE_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(presence_java PROPERTIES BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR})

add_jar(Presence
        java/Presence.java)
get_target_property(_presenceJarFile Presence JAR_FILE)
message(STATUS "Jar file ${_presenceJarFile}")

add_jar(Main examples/Main.java INCLUDE_JARS Presence)
add_dependencies(Main Presence)
get_target_property(_jarFile Main JAR_FILE)
message(STATUS "Jar file ${_jarFile}")


message(STATUS "Load Rust lib from ${CMAKE_CURRENT_BINARY_DIR}")
add_custom_target(presence_java_example ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_jarFile}:${_presenceJarFile} Main)
add_dependencies(presence_java_example Main Presence presence_java)