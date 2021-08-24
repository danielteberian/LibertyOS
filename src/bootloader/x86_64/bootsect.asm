ORG 0x7C00
SECTION .text
USE16
boot:
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7C00
	push ax
	push word .set_cs
	retf
.set_cs:
	mov [disk], dl
	mov si, name
	call print
	call println
	mov bx, (startup_start - boot) / 512
	call print_hex
	call println
	mov bx, startup_start
	call print_hex
	call println
	mov eax, (startup_start - boot) / 512
	mov bx, startup_start
	mov cx, (startup_end - startup_start) / 512
	xor dx, dx
	call load
	call println
	mov si, finished
	call print
	call println
	jmp startup
load:
	cmp cx, 127
	jbe .good_size
	pusha
	mov cx, 127
	call load
	popa
	add eax, 127
	add dx, 127 * 512 / 16
	sub cx, 127
	jmp load
.good_size:
	mov [DAPACK.addr], eax
	mov [DAPACK.buf] bx
	mov [DAPACK.count] cx
	mov [DAPACK.seg] dx
	call print_dapack
	mov dl, [disk]
	mov si, DAPACK
	mov ah, 0x42
	int 0x13
	jc err
	ret
print_dapack:
	mov al, 13
	call print_char
	mov bx, [DAPACK.addr + 2]
	call print_hex
	mov bx, [DAPACK.addr]
	call print_hex
	mov al, '#'
	call print_char
	mov bx, [DAPACK.count]
	call print_hex
	mov al, ' '
	call print_char
	mov bx, [DAPACK.seg]
	call print_hex
	mov al, ':'
	call print_char
	mov bx, [DAPACK.buf]
	call print_hex
	ret
err:
	call println
	mov bh, 0
	mov bl, ah
	call print_hex
	mov al, ' '
	call print_char
	mov si, errored
	call print
	call println
.halt:
	cli
	hlt
	jmp .halt
%include "print.asm"
name: db "LIBERTYOS LOADER: INITIAL STAGE",0
errored: db "[ERR] FAILED TO READ DISK",0
finished: db "LIBERTYOS LOADER: SECONDARY STAGE",0
disk: db 0
DAPACK:
	db 0x10
	db 0
.count: dw 0
.buf: 	dw 0
.seg:	dw 0
.addr:	dq 0
times 446-($-$$) db 0
partitions: times 4 * 16 db 0
db 0x55
db 0xaa
