ORG 0x7c00
BITS 16

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

jmp short init

init:
    jmp 0:start

start:
    cli         ; Disable Interrupts
    xor ax, ax  ; offset zero
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00 ; sp should point to start address(not offset)
    sti         ; Enable Interrupts

.switch_protected:
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp CODE_SEG:load_protected

; GDT
gdt_start:
gdt_null: ; Null 
    dd 0x0
    dd 0x0
; offset 0x8
gdt_code:       ; Code segment
    dw 0xffff   ; Segment Limit
    dw 0        ; Base Address
    db 0        ; Base 23:16
    db 0x9a     ; P-DPL-S-Type
    db 11001111b ; G-D/B-L-AVL-19:16 Segment Limit
    db 0        ; Base 31:24

; offset 0x10
gdt_data:       ; Data Segment DS, SS, ES, FS, GS
    dw 0xffff ; Segment Limit
    dw 0        ; Base Address
    db 0        ; Base 23:16
    db 0x92     ; P-DPL-S-Type
    db 11001111b ; G-D/B-L-AVL-19:16 Segment Limit
    db 0        ; Base 31:24
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start
    

[BITS 32]
load_protected:
    jmp $

times 510-($-$$) db 0
dw 0xAA55
