global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 4
	mov qword r10, 4
	cmp rbx, r10
	jle .L1
	mov rbx, 0
	jmp .L2
.L1:
	mov rbx, 1
.L2:
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
	mov qword [rbp-8], rbx
	mov qword rbx, 4
	mov qword r12, 4
	cmp rbx, r12
	jl .L5
	mov rbx, 0
	jmp .L6
.L5:
	mov rbx, 1
.L6:
	mov qword [rbp-8], rbx
	push 0
	mov qword rbx, 3
	mov qword [rbp-16], rbx
	push 0
	mov rbx, [rbp-16]
	mov qword r13, 5
	cmp rbx, r13
	jl .L7
	mov rbx, 0
	jmp .L8
.L7:
	mov rbx, 1
.L8:
	mov qword [rbp-24], rbx
	mov rbx, [rbp-24]
	mov rdi, rbx
	mov rax, 60
	syscall
