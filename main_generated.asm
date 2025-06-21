global _start
_start:
	push rbp
	mov rbp, rsp
	mov rbx, 3
	push 0
	mov qword [rbp-8], rbx
	push 0
	mov qword [rbp-16], rbx
	mov rdi, rbx
	mov rax, 60
	syscall
