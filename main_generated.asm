global main
global foo
global return_false
foo:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	push rbx
	push r12
	push r13
	push r14
	push r15
	mov qword rax, 3
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
return_false:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	push rbx
	push r12
	push r13
	push r14
	push r15
	mov qword rax, 0
	mov rax, 0
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	mov rsp, rbp
	pop rbp
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	push rbx
	push r12
	push r13
	push r14
	push r15
	mov qword rbx, 1
	mov qword [rbp-8], rbx
	call foo
	mov qword [rbp-8], rax
	call return_false
	mov qword [rbp-16], rax
	mov rax, [rbp-16]
	xor rax, 1
	cmp rax, 0
	je .L1
	mov rbx, [rbp-8]
	mov qword rcx, 1
	add rbx, rcx
	mov qword [rbp-8], rbx
	jmp .L1
.L1:
	mov rbx, [rbp-8]
	mov rax, rbx
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	mov rsp, rbp
	pop rbp
	ret
