global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 4
	mov qword r10, 5
	add rbx, r10
	mov qword r11, 3
	sub r10, r11
	mov qword r12, 3
	sub r11, r12
	mov qword [rbp-8], r10
	mov rdi, r10
	mov rax, 60
	syscall
