cmake_minimum_required(VERSION 3.14...3.28)
project(presence_android)

add_subdirectory( ../../..)

add_library(presence_java SHARED IMPORTED)
set_target_properties(presence_java
        PROPERTIES IMPORTED_LOCATION
        ../../../build/ libpresence_java.so)