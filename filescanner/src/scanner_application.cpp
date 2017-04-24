#include <stdio.h>

extern "C" {
    extern void scan_interface(const char *);
}

int main(int argc, char* argv[])
{
    if(argc > 1)
       scan_interface(argv[1]);
}
