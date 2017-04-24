#include <stdio.h>
#include <stdlib.h>

extern void callback(uint32_t);

int main()
{
    callback(10);
}
