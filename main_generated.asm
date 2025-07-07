global foo
global main
foo:
	push rbp
	mov rbp, rsp
	push rbx
	push r12
	push r13
	push r14
	push r15
	push 0
	mov qword rbx, 3
	mov qword [rbp-8], rbx
	mov qword rbx, 1
	mov rax, 1
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
	push rbx
	push r12
	push r13
	push r14
	push r15
	push 0
	mov qword r10, 1
	mov qword [rbp-8], r10
	call foo
	mov r10, [rbp-8]
	mov rax, r10
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	mov rsp, rbp
	pop rbp
	ret
