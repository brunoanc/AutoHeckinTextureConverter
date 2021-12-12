# Project metadata
cmake_minimum_required(VERSION 3.12)
project(AutoHeckinTextureConverter)
set(CMAKE_CXX_STANDARD 17)

# Source files
file(GLOB SOURCES
        ./main.cpp
        ./utils.cpp
        ./utils.hpp
        )

# Compile arguments
if (MSVC)
        add_compile_definitions(_CRT_SECURE_NO_WARNINGS)
        set(CMAKE_CXX_FLAGS "/O2 /Oi /Ot /GL /EHsc")
else()
        set (CMAKE_CXX_FLAGS "-O2")
endif()

# Add AutoHeckinTextureConverter executable
add_executable(AutoHeckinTextureConverter ${SOURCES})

# use LTCG linking on MSVC
if(MSVC)
        target_link_options(AutoHeckinTextureConverter PUBLIC "/LTCG")
endif()
