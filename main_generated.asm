global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	push rbx
	push r12
	push r13
	push r14
	push r15
	mov qword rax, 3
	mov qword rbx, 2
	mov qword rcx, 4
	mov rax, rbx
	imul rcx
	mov rbx, rax
	add rax, rbx
	mov qword rbx, 3
	mov qword rcx, 4
	mov rax, rbx
	imul rcx
	mov rbx, rax
	add rax, rbx
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	mov rax, rax
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	mov rsp, rbp
	pop rbp
	ret
