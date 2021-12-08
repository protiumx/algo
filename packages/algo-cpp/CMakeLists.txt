cmake_minimum_required(VERSION 3.14)
project(algo-cpp)

# GoogleTest requires at least C++11
set(CMAKE_CXX_STANDARD 11)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/3e0e32ba300ce8afe695ad3ba7e81b21b7cf237a.zip
)
FetchContent_MakeAvailable(googletest)

# file(GLOB_RECURSE sources ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp)

add_executable(tests main.cpp algo_00.cpp algo_01.cpp)

target_link_libraries(tests gtest_main)
enable_testing()
include(GoogleTest)
gtest_discover_tests(tests)
