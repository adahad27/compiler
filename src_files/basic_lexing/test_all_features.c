int main() {
    int x;
    for(int i = 0; i < 3; ++i) {
        x = x + i;
    }

    if(x > 1) {
        x = x - 1;
    }
    else if (x > 2) {
        x = x - 2;
    }
    else {
        x = x - 3;
    }
}