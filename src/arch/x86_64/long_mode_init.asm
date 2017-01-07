global long_mode_start

section .text
bits 64 ;declaring 64bit instruction
long_mode_start:
  ; call the rust main
  extern rust_main
  call rust_main

  ; print "OKAY" to the screen
  mov rax, 0x2f592f412f4b2f4f ;use rax/rbx instead of eax/ebx
  mov qword [0xb8000], rax
  hlt