global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov dword [rbp-8], 3
	mov dword [rbp-8], 5
	push 0
	mov dword [rbp-16], 4
	mov rdi, [rbp-16]
	mov rax, 60
	syscall
