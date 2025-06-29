int main() {
    int x = 0;
    int i = 0;

    while(i < 5) {
        x = x + i;
        i = i + 1;
    }

    for(int j = 0; j < 5; j = j + 1) {
        x = x + j;
    }


    return x;
}