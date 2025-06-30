global main
main:
	push rbp
	mov rbp, rsp
	push 3
	push 0
	mov rax, [rbp-8]
	mov rsp, rbp
	pop rbp
	ret
