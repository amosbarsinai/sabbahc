.section .text
	.globl _start
	_start:
		call main        # Call `main` function from user's code
		mov %rax, %rdi   # `main` function returns exit code in an 8-bit unsigned integer in RAX - move to RDI for exit syscall
		mov $0x60, %rax  # 60 - exit syscall on Linux x86_64
		syscall          # Request exit service from kernel
