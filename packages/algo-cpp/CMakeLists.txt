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


enable_testing()
add_executable(
  algo_00
  algo_00.cpp
)
target_link_libraries(
  algo_00
  gtest_main
)

include(GoogleTest)
gtest_discover_tests(algo_00)
