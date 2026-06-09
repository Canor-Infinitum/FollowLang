cmake_minimum_required(VERSION 3.16)
project(infinitum_shared C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

add_library(infinitum_shared SHARED
    novus.c
    tempus.c
    canor.c
    aether.c
    envoy.c
    gui.c
    infinitum.c
)

target_include_directories(infinitum_shared PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_definitions(infinitum_shared PRIVATE INFINITUM_SHARED_BUILD=1)
set_target_properties(infinitum_shared PROPERTIES OUTPUT_NAME "infinitum.shared")