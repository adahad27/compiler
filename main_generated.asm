global return_false
global main
global foo
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
	sub rsp, 16
	mov qword rbx, 1
	mov qword [rbp-8], rbx
	mov rbx, [rbp-8]
	push rbx
	call foo
	add rsp, 8
	mov qword [rbp-8], rax
	call return_false
	add rsp, 0
	mov qword [rbp-16], rax
	call return_false
	add rsp, 0
	xor rax, 1
	cmp rax, 0
	je .L1
	mov rax, [rbp-8]
	mov qword rcx, 1
	add rax, rcx
	mov qword [rbp-8], rax
	jmp .L1
.L1:
	mov rax, [rbp-8]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
