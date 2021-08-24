struct mbr_partition_rec
.sys: resb 1
.chs_start: resb 3
.ty: resb 1
.chs_end: resd 3
.lba_start: resd 1
.sect_count: resd 1
endstruc
find_libfs_partition:
	xor ebx, ebx
.loop:
	mov al, byte [partitions + mbr_partition_rec + mbr_partition_rec.ty]
	cmp al, 0x83
	je .found
	add ebx, 1
	cmp ebx, 4
	jb .loop
	jmp .notfound
.found:
	mov eax, [partitions + mbr_partitions_rec + mbr_partitions_rec.lba_start]
	ret
.notfound:
	mov si, .no_partmsg
	call print
	mov eax, (filesystem - boot) / 512
	ret
.not_partmsg: db "[ERR] COULD NOT FIND A MBR PARTITION WITH 0x83 TYPE", 0xA, 0xD, 0
