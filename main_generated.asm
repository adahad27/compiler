global _start
_start:
	sub rsp, 4
	push 3
	sub rsp, 4
	push 0
	mov rdi, 69
	mov rax, 60
	syscall
