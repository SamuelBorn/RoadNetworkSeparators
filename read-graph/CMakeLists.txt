# Subdirectory for the read-graph subproject
project(readgraph)

# Gather all .cpp files
file(GLOB READ_GRAPH_SOURCES *.cpp)

# Create a library from the subproject
add_library(readgraph ${READ_GRAPH_SOURCES})

# Include the current directory for headers
target_include_directories(readgraph PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

cmake_minimum_required(VERSION 3.16)
project(ReadGraph)

# Create a static library from the cpp files
file(GLOB_RECURSE READ_GRAPH_SRC CONFIGURE_DEPENDS *.cpp)
add_library(read_graph_lib STATIC ${READ_GRAPH_SRC})

# Include directory for vector_io.h
target_include_directories(read_graph_lib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

