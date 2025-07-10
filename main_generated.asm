global main
global foo
foo:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov rax, [rbp+8]
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	mov qword rbx, 3
	add rax, rbx
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword rax, 1
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	push rax
	call foo
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
