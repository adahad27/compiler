global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword rax, 0
	mov qword [rbp-8], rax
	mov qword rax, 0
	mov qword [rbp-16], rax
.L1:
	mov rax, [rbp-16]
	mov qword rbx, 5
	cmp rax, rbx
	jl .L3
	mov rax, 0
	jmp .L4
.L3:
	mov rax, 1
.L4:
	cmp rax, 0
	je .L2
	mov rcx, [rbp-8]
	mov rdx, [rbp-16]
	add rcx, rdx
	mov qword [rbp-8], rcx
	mov rcx, [rbp-16]
	mov qword rdx, 1
	add rcx, rdx
	mov qword [rbp-16], rcx
	jmp .L1
.L2:
	mov qword rcx, 0
	mov qword [rbp-24], rcx
.L5:
	mov rcx, [rbp-24]
	mov qword rdx, 5
	cmp rcx, rdx
	jl .L7
	mov rcx, 0
	jmp .L8
.L7:
	mov rcx, 1
.L8:
	cmp rcx, 0
	je .L6
	mov rsi, [rbp-8]
	mov rdi, [rbp-24]
	add rsi, rdi
	mov qword [rbp-8], rsi
	mov rsi, [rbp-24]
	mov qword rdi, 1
	add rsi, rdi
	mov qword [rbp-24], rsi
	jmp .L5
.L6:
	mov rsi, [rbp-8]
	mov rdi, [rbp-16]
	add rsi, rdi
	mov qword [rbp-8], rsi
	mov rsi, [rbp-8]
	mov rax, rsi
	mov rsp, rbp
	pop rbp
	ret
