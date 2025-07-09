global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	push 0
	mov qword rbx, 1
	mov qword [rbp-16], rbx
	push 0
	mov qword rbx, 0
	mov qword [rbp-24], rbx
	mov rbx, [rbp-16]
	mov r10, [rbp-24]
	xor r10, 1
	mov r11, [rbp-24]
	cmp r10, 0
	je .L1
	cmp r11, 0
	je .L1
	mov r10, 1
	jmp .L2
.L1:
	mov r10, 0
.L2:
	cmp rbx, 1
	je .L3
	cmp r10, 1
	je .L3
	mov rbx, 0
	jmp .L4
.L3:
	mov rbx, 1
.L4:
	mov r10, [rbp-16]
	mov r11, [rbp-24]
	cmp r10, 0
	je .L5
	cmp r11, 0
	je .L5
	mov r10, 1
	jmp .L6
.L5:
	mov r10, 0
.L6:
	cmp rbx, 1
	je .L7
	cmp r10, 1
	je .L7
	mov rbx, 0
	jmp .L8
.L7:
	mov rbx, 1
.L8:
	mov qword [rbp-8], rbx
	mov rbx, [rbp-8]
	mov rdi, rbx
	mov rax, 60
	syscall
