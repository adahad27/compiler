global _start
_start:
	push rbp
	mov rbp, rsp
	mov qword rbx, 2
	push 0
	mov qword [rbp-8], rbx
	mov qword rbx, 4
	push 0
	mov qword [rbp-16], rbx
	mov qword rbx, 3
	push 0
	mov qword [rbp-24], rbx
	mov rbx, [rbp-24]
	mov qword r10, 2
	add rbx, r10
	mov r10, [rbp-16]
	add rbx, r10
	mov qword r10, 1
	add rbx, r10
	mov r10, [rbp-8]
	add rbx, r10
	push 0
	mov qword [rbp-32], rbx
	mov rbx, [rbp-32]
	mov rdi, rbx
	mov rax, 60
	syscall
