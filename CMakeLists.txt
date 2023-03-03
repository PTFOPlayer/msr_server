cmake_minimum_required(VERSION 3.16.3)

project(msr_server)

add_executable(msr_gen ./src/main.cpp)

execute_process(COMMAND sudo cp msr_gen /usr/bin/)

execute_process(COMMAND (sudo cp ../msr_server.service /etc/systemd/system/) || (sudo cp ./msr_server.service /etc/systemd/system/))