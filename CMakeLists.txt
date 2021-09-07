cmake_minimum_required(VERSION 3.0)

project(quetzal VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_executable(quetzal "src/quetzal.cpp")

include_directories(quetzal "${PROJECT_SOURCE_DIR}/lib")
