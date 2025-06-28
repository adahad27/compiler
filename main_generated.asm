global _start
_start:
	push rbp
	mov rbp, rsp
	mov qword rbx, 2
	push 0
	mov qword [rbp-8], rbx
	mov qword r10, 3
	push 0
	mov qword [rbp-16], r10
	push 0
	mov qword r11, 2
	mov qword r12, 3
	add r11, r12
	mov rdi, r11
	mov rax, 60
	syscall
