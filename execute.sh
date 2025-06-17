nasm -felf64 main_generated.asm
ld main_generated.o
./a.out
echo $?
rm main_generated.o a.out