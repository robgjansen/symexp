#include <stdio.h>

//extern void external();

// NOT externally visible
void local() {
    printf("inside shim::local() now\n");
}

int sleep(unsigned int s) {
    printf("inside shim::sleep(%u) now\n", s);
    local();
    // external();
    return 654321;
}
