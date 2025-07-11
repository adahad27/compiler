global main
main:
	push rbp
	mov rbp, rsp
	sub rsp, 24
	mov qword rax, 4
	mov qword rbx, 4
	cmp rax, rbx
	jle .L1
	mov rax, 0
	jmp .L2
.L1:
	mov rax, 1
.L2:
	mov qword [rbp-8], rax
	mov qword rax, 4
	mov qword rcx, 4
	cmp rax, rcx
	jge .L3
	mov rax, 0
	jmp .L4
.L3:
	mov rax, 1
.L4:
	mov qword [rbp-8], rax
	mov qword rax, 4
	mov qword rdx, 4
	cmp rax, rdx
	jl .L5
	mov rax, 0
	jmp .L6
.L5:
	mov rax, 1
.L6:
	mov qword [rbp-8], rax
	mov qword rax, 3
	mov qword [rbp-16], rax
	mov rax, [rbp-16]
	mov qword rsi, 5
	cmp rax, rsi
	jl .L7
	mov rax, 0
	jmp .L8
.L7:
	mov rax, 1
.L8:
	mov qword [rbp-24], rax
	mov rax, [rbp-24]
	mov rax, rax
	mov rsp, rbp
	pop rbp
	ret
