#include <stdlib.h>
int main() {

    int* some_ptr = malloc(4);
    *some_ptr = 3;

    free(some_ptr);

    return 0;


}