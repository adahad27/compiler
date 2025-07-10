

int foo(int bar) {
    int x = bar;
    return x + 3;
}

// bool return_false() {
//     return 0;
// }

int main(){
    int x = 1;
    x = foo(x);
    // bool y = return_false();
    
    // if(!return_false()) {
    //     x = x + 1;
    // }
    return x;
}