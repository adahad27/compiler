global _start
_start:
	push rbp
	mov rbp, rsp
	mov qword rbx, 4
	mov qword r10, 4
	cmp rbx, r10
	jle .L1
	mov rbx, 0
	jmp .L2
.L1:
	mov rbx, 1
.L2:
	push 0
	mov qword [rbp-8], rbx
	mov qword r11, 4
	mov qword r12, 4
	cmp r11, r12
	jge .L3
	mov r11, 0
	jmp .L4
.L3:
	mov r11, 1
.L4:
	mov qword [rbp-8], r11
	mov qword r13, 4
	mov qword r14, 4
	cmp r13, r14
	jl .L5
	mov r13, 0
	jmp .L6
.L5:
	mov r13, 1
.L6:
	mov qword [rbp-8], r13
	mov rdi, r13
	mov rax, 60
	syscall
