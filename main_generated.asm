global return_false
global foo
global main
foo:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov rax, [rbp+8]
	mov qword [rbp-8], rax
	mov rax, [rbp-8]
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
	sub rsp, 16
	mov qword rbx, 1
	mov qword [rbp-8], rbx
	call foo
	mov qword [rbp-8], rax
	call return_false
	mov qword [rbp-16], rax
	call return_false
	xor rax, 1
	cmp rax, 0
	je .L1
	mov rax, [rbp-8]
	mov qword rbx, 1
	add rax, rbx
	mov qword [rbp-8], rax
	jmp .L1
.L1:
	mov rax, [rbp-8]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
