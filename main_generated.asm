global _start
_start:
	push rbp
	mov rbp, rsp
	mov rbx, 4
	mov r10, 3
	add r10, rbx
	push 0
	mov qword [rbp-8], r10
	mov rdi, r10
	mov rax, 60
	syscall
