global _start
_start:
	push rbp
	mov rbp, rsp
	mov qword rbx, 3
	mov qword r10, 4
	cmp rbx, r10
	jl .L1
	mov rbx, 0
	jmp .L2
.L1:
	mov rbx, 1
.L2:
	push 0
	mov qword [rbp-8], rbx
	mov qword r11, 0
	mov rdi, 0
	mov rax, 60
	syscall
