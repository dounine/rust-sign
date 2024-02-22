#include "hello.hpp"
#include <iostream>

const char* get_hello(char *error) {
    std::cout << "你好呀，动态库!" << std::endl;
    snprintf(error, 1024, "this is error");
    return "你好呀，动态库!";
}