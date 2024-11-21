cmake_minimum_required(VERSION 3.14...3.28)
project(presence_java)

find_package(Java REQUIRED)
include(UseJava)

if (CMAKE_BUILD_TYPE STREQUAL "Release")
    set(RELEASE_DIR "release")
    set(RELEASE "--release")
else ()
    set(RELEASE_DIR "debug")
endif ()
if (ANDROID_HOME_DIR)
    message(STATUS "Set cargo for Android ${ANDROID_HOME_DIR}.")
    set(TARGET aarch64-linux-android)
    # set(CARGO_CMD ANDROID_HOME=${ANDROID_HOME_DIR} cargo ndk --platform 30 --target ${TARGET} build --features mock ${RELEASE})
    set(CARGO_CMD ANDROID_HOME=${ANDROID_HOME_DIR} cargo ndk --platform 30 --target ${TARGET} build ${RELEASE})
    set(PRESENCE_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET}/${RELEASE_DIR}/libpresence_java.so")
else ()
    message(STATUS "Set cargo for Linux.")
    set(CARGO_CMD cargo build  --features mock ${RELEASE})
    set(PRESENCE_SO "${CMAKE_CURRENT_BINARY_DIR}/${RELEASE_DIR}/libpresence_java.so")
endif ()

message(STATUS "Cargo build Presence Java.")
add_custom_target(presence_java ALL
    COMMENT "Compiling Presence Java module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${PRESENCE_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(presence_java PROPERTIES BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR})

if (ANDROID_HOME_DIR)
    return ()
endif ()

add_jar(Presence
        java/DiscoveryResult.java
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
        -cp ${_jarFile}:${_presenceJarFile} -ea Main)
add_dependencies(presence_java_example Main Presence presence_java)

add_jar(DiscoveryResult
        java/DiscoveryResult.java
        examples/TestDiscoveryResult.java)
get_target_property(_discoveryResultJarFile DiscoveryResult JAR_FILE)
message(STATUS "Jar file ${_discoveryResultJarFile}")


set(TEST_DISCOVERY_RESULT_SO "${CMAKE_CURRENT_BINARY_DIR}/debug/examples/libtest_discovery_result.so")
message(STATUS "Cargo build ${TEST_DISCOVERY_RESULT_SO}")
add_custom_target(test_discovery_result_so ALL
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} cargo build --examples
    COMMAND cp ${TEST_DISCOVERY_RESULT_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

message(STATUS "Test FFI discovery result.")
add_custom_target(test_discovery_result ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=${CMAKE_CURRENT_BINARY_DIR}
        -cp ${_discoveryResultJarFile} -ea TestDiscoveryResult)
add_dependencies(test_discovery_result DiscoveryResult test_discovery_result_so)