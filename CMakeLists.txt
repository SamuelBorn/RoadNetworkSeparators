cmake_minimum_required(VERSION 3.10)

project(graph_seperator_size)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(${PROJECT_NAME} main.cpp my_graph_library.cpp)

target_include_directories(
  ${PROJECT_NAME} PUBLIC ${CMAKE_SOURCE_DIR}/KaHIP/deploy
                         ${CMAKE_SOURCE_DIR}/read-graph)

target_link_libraries(${PROJECT_NAME}
                      ${CMAKE_SOURCE_DIR}/KaHIP/deploy/libkahip.so)
