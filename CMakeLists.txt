cmake_minimum_required(VERSION 3.12)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
project(msr_server C CXX ASM)

set(CMAKE_C_FLAGS "${CMAKE_CXX_FLAGS} -lm")



add_executable(msr_gen ./src/main.c)

target_link_libraries(
    msr_gen
    ${CMAKE_SOURCE_DIR}/target/release/libmsr_rs.a
)