global _start
_start:
	push rbp
	mov rbp, rsp
	push 3
	push 0
	mov rdi, [rbp-16]
	mov rax, 60
	syscall
