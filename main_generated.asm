global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 32
	mov qword rax, 2
	mov qword [rbp-8], rax
	mov qword rax, 4
	mov qword [rbp-16], rax
	mov qword rax, 3
	mov qword [rbp-24], rax
	mov rax, [rbp-24]
	mov qword rbx, 2
	add rax, rbx
	mov rbx, [rbp-16]
	add rax, rbx
	mov qword rbx, 1
	add rax, rbx
	mov rbx, [rbp-8]
	add rax, rbx
	mov qword [rbp-32], rax
	mov rax, [rbp-32]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
