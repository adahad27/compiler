global main
main:
	push rbp
	mov rbp, rsp
	push 0
	mov qword rbx, 0
	mov qword [rbp-8], rbx
	push 0
	mov qword rbx, 1
	mov qword [rbp-16], rbx
	mov qword rbx, 0
	cmp rbx, 0
	je .L2
	push 0
	mov qword r10, 0
	mov qword [rbp-24], r10
	mov qword r10, 0
	mov qword [rbp-8], r10
	jmp .L1
.L2:
	mov qword r10, 1
	cmp r10, 0
	je .L3
	push 0
	mov qword r11, 0
	mov qword [rbp-24], r11
	mov qword r11, 1
	mov qword [rbp-8], r11
	jmp .L1
.L3:
	mov qword r11, 0
	mov qword [rbp-8], r11
.L1:
	mov r11, [rbp-16]
	mov rax, r11
	mov rsp, rbp
	pop rbp
	ret
