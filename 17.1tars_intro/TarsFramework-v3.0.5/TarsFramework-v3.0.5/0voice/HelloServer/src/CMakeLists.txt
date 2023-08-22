cmake_minimum_required(VERSION 2.8)

project(TestApp-HelloServer)

gen_server(TestApp HelloServer)

#target_link_libraries(mysqlclient ssl crypto nghttp2-static)
