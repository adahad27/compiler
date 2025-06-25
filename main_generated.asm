global _start
_start:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, true
	push 0
	mov qword [rbp-16], rbx
	mov qword r10, false
	push 0
	mov qword [rbp-24], r10
	mov qword r11, true
	mov qword r12, false
	not r12
	mov qword r13, false
	and r12, r13
	or r11, r12
	mov qword r14, true
	mov qword r15, false
	and r14, r15
	or r11, r14
	mov qword [rbp-8], r11
	mov rdi, rbx
	mov rax, 60
	syscall
