cmake_minimum_required(VERSION 3.0)

project(quetzal VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_CXX_FLAGS_DEBUG_INIT "-Wall -Wextra -Wpedantic -Og -g3 \
-fsanitize=address -Wuseless-cast -Wcast-align -Wcast-qual -Wduplicated-branches \
-Wduplicated-cond -Wempty-body -Wfloat-equal -Wformat=2 -Winit-self -Wmissing-braces")
set(CMAKE_CXX_FLAGS_RELEASE_INIT "-Wall -Wdisabled-optimization -O3 -DNDEBUG")

include_directories(quetzal "${PROJECT_SOURCE_DIR}/lib")

add_executable(qtz
	${PROJECT_SOURCE_DIR}/src/quetzal.cpp
	${PROJECT_SOURCE_DIR}/src/common.cpp
	${PROJECT_SOURCE_DIR}/src/error.cpp
	${PROJECT_SOURCE_DIR}/src/lexer.cpp
	${PROJECT_SOURCE_DIR}/src/parser.cpp
	${PROJECT_SOURCE_DIR}/src/interpreter.cpp
)
