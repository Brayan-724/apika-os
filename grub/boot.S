; bits 32  ;nasm directive - 32 bit
bits 64
section .text
 ;multiboot spec
 align 4
 dd 0x1BADB002 ;magic
 dd 0x00 ;flags
 dd - (0x1BADB002 + 0x00) ;checksum
 
global start ; to set symbols from source code as global 
extern _start ; _start is the function in Rust file
 
start:
 cli ;clear interrupts-- to diable interrupts
 mov esp, stack_space ;set stack pointer
 call _start ;calls the main kernel function from c file
 hlt ;halts the CPU
 
section .bss
resb 8192 ;8KB memory reserved for the stack
stack_space:  
