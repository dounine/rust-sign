cmake_minimum_required(VERSION 3.26)
project(lib_shared C)

set(CMAKE_C_STANDARD 11)

add_library(lib_shared SHARED hello.c)
