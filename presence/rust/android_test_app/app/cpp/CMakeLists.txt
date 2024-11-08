cmake_minimum_required(VERSION 3.22.1)

project("test-ndk")

add_library(test-ndk SHARED
        test-ndk.cpp)

# Include libraries needed for test-ndk lib
target_link_libraries(test-ndk android log)

add_subdirectory(presence_java)

add_library(presence_android SHARED IMPORTED)
set_target_properties(presence_android
        PROPERTIES IMPORTED_LOCATION
        presence_java/build/libpresence_java.so)
