section .text
    global main

main:
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, msg_len
    syscall

section .rodata
    msg: db "<(o )___", 10, " ( \_> /", 10, '  "~~~"', 10
    msg_len: EQU $ - msg
