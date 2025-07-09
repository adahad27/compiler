int main() {
    bool x = false;
    bool y = true;
    if(false) {
        bool y = false;
        x = false;
    }
    elif(true){
        bool y = false;
        x = true;
    }
    else {
        x = false;
    }

    return y;
}