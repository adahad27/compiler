global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 24
	push rbx
	push r12
	push r13
	push r14
	push r15
	mov qword rax, 2
	mov qword [rbp-8], rax
	mov qword rax, 4
	mov qword [rbp-16], rax
	mov qword rax, 3
	mov qword [rbp-24], rax
	mov rax, [rbp-24]
	mov rax, rax
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	mov rsp, rbp
	pop rbp
	ret
