global main
global foo
foo:
	push rbp
	mov rbp, rsp
	mov qword rbx, 1
	mov rax, 1
	mov rsp, rbp
	pop rbp
	ret
main:
	push rbp
	mov rbp, rsp
	mov qword r10, 0
	mov rax, 0
	mov rsp, rbp
	pop rbp
	ret
