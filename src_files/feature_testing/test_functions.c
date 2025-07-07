

int foo() {
    int x = 3;
    return x;
}

bool return_false() {
    return 0;
}

int main(){
    int x = 1;
    x = foo();
    bool y = return_false();
    
    if(!y) {
        x = x + 1;
    }
    return x;
}