#include "librarycpp.h"
#include <iostream>
#include <stdio.h>
//#include <openssl/sha.h>

char* hello_for_cpp() {
    std::cout << "Hello from C++" << std::endl;
    return "你好hello cpp";
}
