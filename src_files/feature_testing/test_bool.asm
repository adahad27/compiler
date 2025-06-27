global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 1
	push 0
	mov qword [rbp-16], rbx
	mov qword r10, 0
	push 0
	mov qword [rbp-24], r10
	mov qword r11, 1
	mov qword r12, 0
	xor r12, 1
	mov qword r13, 0
	cmp r12, 0
	je .L1
	cmp r13, 0
	je .L1
	mov r12, 1
	jmp .L2
.L1:
	mov r12, 0
.L2:
	cmp r11, 1
	je .L3
	cmp r12, 1
	je .L3
	mov r11, 0
	jmp .L4
.L3:
	mov r11, 1
.L4:
	mov qword r14, 1
	mov qword r15, 0
	cmp r14, 0
	je .L5
	cmp r15, 0
	je .L5
	mov r14, 1
	jmp .L6
.L5:
	mov r14, 0
.L6:
	cmp r11, 1
	je .L7
	cmp r14, 1
	je .L7
	mov r11, 0
	jmp .L8
.L7:
	mov r11, 1
.L8:
	mov qword [rbp-8], r11
	mov rdi, r11
	mov rax, 60
	syscall
