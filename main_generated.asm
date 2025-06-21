global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov rbx, 4
	mov qword [rbp-8], rbx
	mov rdi, rbx
	mov rax, 60
	syscall
