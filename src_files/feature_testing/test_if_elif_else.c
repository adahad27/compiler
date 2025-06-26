#include <stdbool.h>
int main() {
    bool x = false;
    bool y = false;

    if(x) {
        return 4;
    }
    else if(y) {
        return 5;
    }
    else{
        return 6;
    }

    return 0;
}