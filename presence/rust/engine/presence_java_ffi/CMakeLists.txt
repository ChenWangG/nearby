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

set(PRESENCE_ENGINE_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libpresence_java.so")

message(STATUS "Cargo build Presence Engine Java FFI.")
add_custom_target(presence_java_ffi ALL
    COMMENT "Compiling Presence Engine Java FFI module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${PRESENCE_ENGINE_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(presence_java_ffi PROPERTIES BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR})

add_jar(PresenceEngine java/Engine.java java/PresenceDiscoveryResult.java)
get_target_property(_engineJarFile PresenceEngine JAR_FILE)
message(STATUS "Jar file ${_engineJarFile}")

add_jar(Main examples/Main.java INCLUDE_JARS PresenceEngine)
add_dependencies(Main PresenceEngine)
get_target_property(_jarFile Main JAR_FILE)
message(STATUS "Jar file ${_jarFile}")

add_custom_target(presence_java_example ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${PRESENCE_DIR}
        -cp ${_jarFile}:${_engineJarFile} Main)
add_dependencies(presence_java_example Main PresenceEngine presence_java_ffi)