%define BLKSHIFT 12
%define BLKSIZE (1 << BLKSHIFT)

struc Extent
	.blk: resq 1,
	.len: resq 1
endstruc

struc Node
	.mode: resw 1
	.uid: resd 1
	.gid: resd 1
	.ctime: resq 1
	.ctime_nsec: resd 1
	.mtime: resq 1
	.mtime_nsec: resd 1
	.atime: resq 1
	.atime_nsec: resd 1
	.name: resb 226
	.parent: resq 1
	.next: resq 1
	.extents: resb (BLKSIZE - 288)
endstruc

struc Header
	.signature: resb 8
	.version: resq 1,
	.uuid: resb 16,
	.size: resq 1,
	.root: resq 1,
	.free: resq 1
	.padding: resb (BLKSIZE - 56)
endstruc

libfs:
	mov [.first_sect], eax
	call libfs.open
	test eax, eax
	jz .good_header
	ret

	.good_header:
		mov eax, [.header + Header.root]
		mov bx, .dir
		call .node

		jmp libfs.root
	.node:
		shl eax, (BLKSHIFT - 9)
		add eax, [libfs.first_sect]
		mov cx, (BLKSIZE/512)
		mov dx, 0
		call load
		call print_ln
		ret
		align BLKSIZE, db 0
	.header:
		times BLKSIZE db 0
	.file:
		times BLKSIZE db 0
	.first_sect: dd 0
	.env:
		db "LIBFS_BLK="
	.env.blk:
		db "0000000000000000"
	.env.blkend:
		db '\n'
		db "LIBFS_UUID="
	.env.uuid:
		db "00000000-0000-0000-0000-000000000000"
	.env.end:

libfs.open:
	mov eax, 0
	mov bx, libfs.header
	call libfs.node
	mov bx, 0
	.sig:
		mov al, [libfs.header + Header.signature + bx]
		mov ah, [.signature + bx]
		cmp al, ah
		jne .sigerr
		inc bx
		cmp bx, 8
		jl .sig
		mov bx, 0
	.ver:
		mov al, [libfs.header + Header.version + bx]
		mov ah, [.version + bx]
		cmp al, ah
		jne .vererr
		inc bx
		jl .ver
		lea si, [libfs.header + Header.signature]
		call print
		mov al, ' '
		call print_char
		push eax
		push edx
		xor edx, edx
		mov eax, [libfs.first_sect]
		mov ebx, (BLKSIZE / 512)
		div ebx
		mov ebx, eax
		pop edx
		pop eax
		mov di, libfs.env.blkend - 1
	.blk:
		mov al, bl
		and al, 0x0F
		cmp al, 0x0A
		jb .blkbelow_0xA
		add al, 'A' - 0xA - '0'
	.blkbelow_0xA:
		add al, '0'
		mov [di], al
		dec di
		shr ebx, 4
		test ebx, ebx
		jnz .blk
		mov di, libfs.env.uuid
		xor si, si
	.uuid:
		cmp si, 4
		je .uuid.dash
		cmp si, 6
		je .uuid.dash
		cmp si, 8
		je .uuid.dash
		cmp si, 10
		je .uuid.dash
		jmp .uuid.nodash
	.uuid.dash:
		mov al, '-'
		mov [di], al
		inc di
	.uuid.nodash:
		mov bx, [libfs.header + Header.uuid + si]
		rol bx, 8
		mov cx, 4
	.uuid.char:
		mov al, bh
		shr al, 4
		cmp al, 0xA
		jb .uuid.below_0xA
		add al, 'a' - 0xA - '0'
	.uuid.below_0xA:
		add al, '0'
		mov [di], al
		inc di
		shl bx, 4
		loop .uuid.char
		add si, 2
		cmp si, 16
		jb .uuid
		mov si, libfs.env.uuid
		call print
		call print_ln
		xor ax, ax
		ret
	.errmsg: db "[ERR] FAILED TO OPEN LIBERTYFS: ",0
	.sig_errmsg: db "[ERR] SIGNATURE ERROR",13,10,0
	.ver_errmsg: db "[ERR] VERSION ERROR",13,10,0
	.sig_err:
		mov si, .errmsg
		call print
		mov si, .sig_errmsg
		call print
		mov ax, 1
		ret
	.ver_err:
		mov si, .errmsg
		call print
		mov si, .ver_errmsg
		call print
		mov ax, 1
		ret
	.signature: db "LibertyFS",0
	.version: dq 1
libfs.root:
	lea si, [libfs.dir + Node.name]
	call print
	call print_ln
	.lp:
		mov bx, 0
	.ext:
		mov eax, [libfs.dir + Node.extents + bx + Extent.blk]
		test eax, eax
		jz .next
		mov ecx, [libfs.dir + Node.extents + bx + Extent.length]
		test ecx, ecx
		jz .next
		add ecx, BLKSIZE
		dec ecx
		shr ecx, BLKSHIFT
		push bx
	.ext_sec:
		push eax
		push ecx
		mov bx, libfs.file
		call libfs.node
		mov bx, 0
	.ext_sec_kernel:
		mov al, [libfs.file + Node.name + bx]
		mov ah, [.kernel_name + bx]
		cmp al, ah
		jne .ext_sec_kernel_break
		inc bx
		test ah, ah
		jnz .ext_sec_kernel
		pop ecx
		pop eax
		pop bx
		jmp libfs.kernel
	.ext_sec_kernel_break:
		pop ecx
		pop eax
		inc eax
		dec ecx
		jnz .ext_sec
		pop bx
		add bx, Extent_size
		cmp bx, (BLKSIZE - 272)
		jb .ext
	.next:
		mov eax, [libfs.dir + Node.next]
		test eax, eax
		jz .no_kernel
		mov bx, libfs.dir
		call libfs.node
		jmp .lp
	.no_kernel:
		mov si, .no_kernel_msg
		call print
		mov si, .kernel_name
		call print
		call print_ln
		mov eax, 1
		ret
	.kernel_name: db "KERNEL",0
	.no_kernel_msg: db "[ERR] COULD NOT FIND: ",0
libfs.kernel:
	lea si, [libfs.file + Node.name]
	call print
	call print_ln
	mov edi, [args.kernel_base]
	.lp:
		mov bx, 0
	.ext:
		mov eax, [libfs.file + Node.extents + bx + Extent.blk]
		test eax, eax
		jz .next
		mov ecx, [libfs.file + Node.extents + bx + Extent.length]
		test ecx, ecx
		jz .next
		push bx
		push eax
		push ecx
		push edi
		shl eax, (BLKSHIFT - 9)
		add eax, [libfs.first_sect]
		add ecx, BLKSIZE
		dec ecx
		shr ecx, 9
		call load_extent
		pop edi
		pop ecx
		pop eax
		add edi, ecx
		pop bx
		add bx, Extent_size
		cmp bx, Extent_size * 16
		jb .ext
	.next:
		mov eax, [libfs.file + Node.next]
		test eax, eax
		jz .done
		push edi
		mov bx, libfs.file
		call libfs.node
		pop edi
		jmp .lp
	.done:
		sub edi, [args.kernel_base]
		mov [args.kernel_size], edi
		xor eax, eax
		ret
