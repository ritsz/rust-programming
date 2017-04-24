#include <iostream>
#include <string>

extern "C" {
    extern void callback(const char *);
}

int main()
{
    callback("HELLO");
}
