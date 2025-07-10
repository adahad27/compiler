global main
global foo
global return_false
foo:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov rax, [rbp+16]
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	mov qword rbx, 3
	add rax, rbx
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
return_false:
	push rbp
	mov rbp, rsp
	mov qword rax, 0
	mov rax, 0
	mov rsp, rbp
	pop rbp
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword rbx, 1
	mov qword [rbp-8], rbx
	mov rbx, [rbp-8]
	push rbx
	call foo
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
