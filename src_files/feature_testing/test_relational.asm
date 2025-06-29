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
	mov qword rbx, 4
	mov qword r11, 4
	cmp rbx, r11
	jge .L3
	mov rbx, 0
	jmp .L4
.L3:
	mov rbx, 1
.L4:
	mov qword r12, 4
	mov qword r13, 4
	cmp r12, r13
	jl .L5
	mov r12, 0
	jmp .L6
.L5:
	mov r12, 1
.L6:
	mov qword r14, 3
	push 0
	mov qword [rbp-16], r14
	mov r14, [rbp-16]
	mov qword r15, 5
	cmp r14, r15
	jl .L7
	mov r14, 0
	jmp .L8
.L7:
	mov r14, 1
.L8:
	push 0
	mov qword [rbp-24], r14
	mov r14, [rbp-8]
	mov rdi, r14
	mov rax, 60
	syscall
