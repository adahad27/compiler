global _start
_start:
	push rbp
	mov rbp, rsp
	mov qword rbx, 0
	push 0
	mov qword [rbp-8], rbx
	mov qword r10, 0
	cmp r10, 0
	je .L2
	mov qword r11, 0
	mov qword [rbp-8], r11
	jmp .L1
.L2:
	mov qword r12, 1
	cmp r12, 0
	je .L3
	mov qword r13, 1
	mov qword [rbp-8], r13
	jmp .L1
.L3:
	mov qword r14, 0
	mov qword [rbp-8], r14
.L1:
	mov rdi, r14
	mov rax, 60
	syscall
