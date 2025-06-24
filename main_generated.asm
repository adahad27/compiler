global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 6
	mov qword r10, 3
	mov rdx, 0
	mov rax, rbx
	idiv r10
	mov rbx, rax
	mov qword r11, 2
	mov qword r12, 5
	mov rax, r11
	imul r12
	mov r11, rax
	mov qword r13, 6
	mov rax, r11
	imul r13
	mov r11, rax
	add rbx, r11
	mov qword [rbp-8], rbx
	mov rdi, rbx
	mov rax, 60
	syscall
