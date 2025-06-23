global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 4
	mov qword r10, 5
	add rbx, r10
	mov qword r11, 3
	sub rbx, r11
	mov qword r12, 1
	add rbx, r12
	mov qword r13, 2
	add rbx, r13
	mov qword r14, 9
	sub rbx, r14
	mov qword r15, 10
	add rbx, r15
	mov qword [rbp-8], rbx
	mov rdi, rbx
	mov rax, 60
	syscall
