int main() {
    bool x;
    bool y = true;
    bool z = false;

    x = y || !z && z || y && z;

    return x;
}