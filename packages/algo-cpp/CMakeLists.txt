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

include_directories(${PROJECT_SOURCE_DIR})
file(GLOB_RECURSE SRC_FILES ${PROJECT_SOURCE_DIR}/solutions/*.cpp)

add_executable(tests main.cpp ${SRC_FILES})

target_link_libraries(tests gtest_main)
enable_testing()
include(GoogleTest)
gtest_discover_tests(tests)
