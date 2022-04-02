cmake_minimum_required(VERSION 3.18)

project(
    "clox"
    LANGUAGES C
    VERSION "0.1.0"
)

add_executable("clox" "chunk.c" "debug.c" "main.c" "memory.c" "value.c")
