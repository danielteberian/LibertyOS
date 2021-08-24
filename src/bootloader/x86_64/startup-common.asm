SECTION .text
USE16
args:
	.kernel_base dq 0x100000
	.kernel_size dq 0
	.stack_base dq 0
	.stack_size dq 0
	.env_base dq 0
	.env_size dq 0
	.acpi_rsdps_base dq 0
	.acpi_rsdps_size dq 0
startup:
	in al, 0x92
	or al, 2
	out 0x92, al
	%ifdef KERNEL
		mov edi, [args.kernel_base]
		mov ecx, (kernel_file.end - kernel_file)
		mov [args.kernel_size], ecx
		mov eax, (kernel_file - boot)/512
		add ecx, 511
		shr ecx, 9
		call load_extent
	%else
		%ifdef FILESYSTEM
			mov eax, (filesystem - boot) / 512
		%else
			call find_libfs_partition
		%endif
		call libfs
		test eax, eax
		jnz err
	%endif
	jmp .loaded_kernel
.loaded_kernel:
	call check_cpuid
	call memmap
	call vesa
	mov si, init_fpu_msg
	call print
	call init.fpu
	mov si, init_sse_msg
	call print
	call init.sse
	mov si, startup_arch_msg
	call print
	jmp startup_arch
load_extent:
	buffer_size_sectors equ 127
.lp:
	cmp ecx, buffer_size_sectors
	jb .break
	push eax
	push ecx
	push edi
	mov ecx, buffer_size_sectors
	mov bx, startup_end
	mov dx, 0x0
	call load
	call unreal
	pop edi
	mov esi, startup_end
	mov ecx, buffer_size_sectors * 512 / 4
	cld
	a32 rep movsd
	pop ecx
	pop eax
	add eax, buffer_size_sectors
	sub ecx, buffer_size_sectors
	jmp .lp
.break:
	test ecx, ecx
	jz .finish
	push ecx
	push edi
	mov bx, startup_end
	mov dx, 0x0
	call load
	call unreal
	pop edi
	pop ecx
	mov esi, startup_end
	shl ecx, 7
	cld
	a32 rep movsd
.finish:
	call println
	ret
%include "conf.asm"
%include "descflag.inc"
%include "gdt_entry.inc"
%include "unreal.asm"
%include "memmap.asm"
%include "vesa.asm"
%include "init.asm"
%include "cpuid.asm"
%ifndef KERNEL
	%include "libfs.asm"
	%ifndef FILESYSTEM
		%include "partitions.asm"
	%endif
%endif
init_fpu_msg: db "[INIT] FPU",13,10,0
init_sse_msg: db "[INIT] SSE",13,10,0
init_pit_msg: db "[INIT] PIT",13,10,0
init_pic_msg: db "[INIT] PIC",13,10,0
startup_arch_msg: db "STARTING ARCH",13,10,0
