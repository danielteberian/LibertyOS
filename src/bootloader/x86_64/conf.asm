SECTION .text
USE16
align 512, db 0
.conf:
	.xres: dw 0
	.yres: dw 0
times 512 - ($ - conf) db 0
svconf:
	mov eax, (conf - boot) / 512
	mov bx, conf
	mov cx, 1
	xor dx, dx
	call store
	ret
store:
	cmp cx, 127
	jbe .good_size
	pusha
	mov cx, 127
	call store
	popa
	add ax, 127
	add dx, 127 * 512 / 16
	sub cx, 127
	jmp store
.good_size:
	mov [DAPACK.addr], eax
	mov [DAPACK.buf], bx
	mov [DAPACK.count], cx
	mov [DAPACK.seg], dx
	call print_dapack
	mov dl, [disk]
	mov si, DAPACK
	mov ah, 0x43
	int 0x13
	jc err
	ret
