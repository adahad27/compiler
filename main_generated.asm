global _start
_start:
	push rbp
	mov rbp, rsp
	mov rbx, 2
	push 0
	mov qword [rbp-8], rbx
	mov r10, 4
	push 0
	mov qword [rbp-16], r10
	mov r11, 3
	push 0
	mov qword [rbp-24], r11
	mov r12, 1
	add r12, rbx
	add r10, r12
	mov r13, 2
	add r13, r10
	add r11, r13
	push 0
	mov qword [rbp-32], r11
	mov rdi, r11
	mov rax, 60
	syscall
