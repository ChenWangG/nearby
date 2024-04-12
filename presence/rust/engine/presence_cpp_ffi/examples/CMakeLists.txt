include_directories("${CMAKE_SOURCE_DIR}")
set(CMAKE_INCLUDE_CURRENT_DIR ON)
get_target_property(PRESENCE_DIR presence BINARY_DIR)
set(LIB_ENGINE_SO ${PRESENCE_DIR}/libpresence.so)

add_executable(main main.cpp)
target_link_libraries(main ${LIB_ENGINE_SO})
add_dependencies(main presence)