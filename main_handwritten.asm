global _start
_start:
    mov rax, 60 ; %rax is the register that will be checked for sys calls
    mov rdi, 1 ; %rdi is the register that will contain the error code for exit
    syscall