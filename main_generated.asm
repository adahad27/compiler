global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov rbx, 4
	mov qword [rbp-8], 4
	mov r10, [rbp-8]
	mov rdi, r10
	mov rax, 60
	syscall
