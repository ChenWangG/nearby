cmake_minimum_required(VERSION 3.22.1)

project("test-ndk")

#find_package(JNI)
#
#if (JNI_FOUND)
#    message (STATUS "JNI_INCLUDE_DIRS=${JNI_INCLUDE_DIRS}")
#    message (STATUS "JNI_LIBRARIES=${JNI_LIBRARIES}")
#endif()
#include_directories(${JNI_INCLUDE_DIRS})

add_library(test-ndk SHARED
        test-ndk.cpp)

# Include libraries needed for test-ndk lib
target_link_libraries(test-ndk android log)