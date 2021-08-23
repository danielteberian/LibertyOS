SECTION .text
USE16
memory_map:
.start		equ 0x0500
.end		equ 0x5000
.length		equ .end - .start

	xor eax, eax
	mov di, .start
	mov ecx, .length / 4
	cld
	rep stosd

	mov di, .start
	mov edx, 0x534D4150
	xor ebx, ebx
.lp:
	mov eax, 0xE820
	mov ecx, 24

	int 0x15
	jc .done

	cmp ebx, 0
	je .done

	add di, 24
	cmp di, .end
	jb .lp
.done:
	ret
