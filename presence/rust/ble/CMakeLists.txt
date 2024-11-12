cmake_minimum_required(VERSION 3.14...3.28)
project(nearby_ble)

find_package(Java REQUIRED)
include(UseJava)

add_subdirectory(tests)