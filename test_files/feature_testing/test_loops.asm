global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 0
	mov qword [rbp-8], rbx
	push 0
	mov qword rbx, 0
	mov qword [rbp-16], rbx
.L1:
	mov rbx, [rbp-16]
	mov qword r10, 5
	cmp rbx, r10
	jl .L3
	mov rbx, 0
	jmp .L4
.L3:
	mov rbx, 1
.L4:
	cmp rbx, 0
	je .L2
	mov r11, [rbp-8]
	mov r12, [rbp-16]
	add r11, r12
	mov qword [rbp-8], r11
	mov r11, [rbp-16]
	mov qword r12, 1
	add r11, r12
	mov qword [rbp-16], r11
	jmp .L1
.L2:
	push 0
	mov qword r11, 0
	mov qword [rbp-24], r11
.L5:
	mov r11, [rbp-24]
	mov qword r12, 5
	cmp r11, r12
	jl .L7
	mov r11, 0
	jmp .L8
.L7:
	mov r11, 1
.L8:
	cmp r11, 0
	je .L6
	mov r13, [rbp-8]
	mov r14, [rbp-24]
	add r13, r14
	mov qword [rbp-8], r13
	mov r13, [rbp-24]
	mov qword r14, 1
	add r13, r14
	mov qword [rbp-24], r13
	jmp .L5
.L6:
	mov r13, [rbp-8]
	mov rdi, r13
	mov rax, 60
	syscall
