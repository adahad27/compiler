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
	mov r11, [rbp-8]
	mov rdi, r11
	mov rax, 60
	syscall
