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

if (ANDROID_HOME)
    message(STATUS "Set cargo for Android.")
    set(TARGET aarch64-linux-android)
    set(CARGO_CMD ANDROID_HOME=${ANDROID_HOME} cargo ndk --platform 30 --target ${TARGET} build ${RELEASE})
    set(PRESENCE_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET}/${RELEASE_DIR}/libpresence_java.so")
else ()
    message(STATUS "Set cargo for Linux.")
    set(CARGO_CMD cargo build ${RELEASE})
    set(PRESENCE_SO "${CMAKE_CURRENT_BINARY_DIR}/${RELEASE_DIR}/libpresence_java.so")
endif ()

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

if (ANDROID_HOME)
     return ()
endif ()

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