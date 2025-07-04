global foo
global main
foo:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 3
	mov qword [rbp-8], rbx
	mov qword rbx, 1
	mov rax, 1
	mov rsp, rbp
	pop rbp
	ret
main:
	push rbp
	mov rbp, rsp
	push 0
	mov qword r10, 1
	mov qword [rbp-8], r10
	mov r10, [rbp-8]
	mov rax, r10
	mov rsp, rbp
	pop rbp
	ret
