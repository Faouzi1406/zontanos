	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0
	.globl	_println                        ; -- Begin function println
	.p2align	2
_println:                               ; @println
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #32                     ; =32
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh0:
	adrp	x8, l_str_pointer@PAGE
	mov	w9, #10
Lloh1:
	add	x8, x8, l_str_pointer@PAGEOFF
	stp	x0, x9, [sp]
	mov	x0, x8
	bl	_printf
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	add	sp, sp, #32                     ; =32
	ret
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc
                                        ; -- End function
	.globl	_readChar                       ; -- Begin function readChar
	.p2align	2
_readChar:                              ; @readChar
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	_getchar
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_putChar                        ; -- Begin function putChar
	.p2align	2
_putChar:                               ; @putChar
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	.cfi_def_cfa_offset 32
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	w19, w0
	bl	_putchar
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	mov	w0, w19
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_welkom                         ; -- Begin function welkom
	.p2align	2
_welkom:                                ; @welkom
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh2:
	adrp	x0, l_str_pointer.1@PAGE
Lloh3:
	add	x0, x0, l_str_pointer.1@PAGEOFF
	bl	_println
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpAdd	Lloh2, Lloh3
	.cfi_endproc
                                        ; -- End function
	.globl	_recurse                        ; -- Begin function recurse
	.p2align	2
_recurse:                               ; @recurse
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x20, x19, [sp, #-32]!           ; 16-byte Folded Spill
	stp	x29, x30, [sp, #16]             ; 16-byte Folded Spill
	add	x29, sp, #16                    ; =16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	w19, w1
	mov	w20, w0
	mov	w8, #10
Lloh4:
	adrp	x0, l_str_pointer.2@PAGE
Lloh5:
	add	x0, x0, l_str_pointer.2@PAGEOFF
	stp	x20, x8, [sp, #-16]!
	bl	_printf
	add	sp, sp, #16                     ; =16
	cmp	w20, w19
	b.lt	LBB4_2
; %bb.1:                                ; %if_do
Lloh6:
	adrp	x0, l_str_pointer.3@PAGE
Lloh7:
	add	x0, x0, l_str_pointer.3@PAGEOFF
	bl	_println
	mov	w0, w20
	b	LBB4_3
LBB4_2:                                 ; %else_do
	mov	x8, sp
	sub	x9, x8, #16                     ; =16
	mov	sp, x9
	ldur	w9, [x8, #-16]
	mov	w1, w19
	add	w0, w9, #1                      ; =1
	stur	w0, [x8, #-16]
	bl	_recurse
LBB4_3:                                 ; %if_do
	sub	sp, x29, #16                    ; =16
	ldp	x29, x30, [sp, #16]             ; 16-byte Folded Reload
	ldp	x20, x19, [sp], #32             ; 16-byte Folded Reload
	ret
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc
                                        ; -- End function
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	stp	x29, x30, [sp, #-16]!           ; 16-byte Folded Spill
	.cfi_def_cfa_offset 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w1, #1000
	mov	w0, wzr
	bl	_recurse
Lloh8:
	adrp	x0, l_str_pointer.4@PAGE
Lloh9:
	add	x0, x0, l_str_pointer.4@PAGEOFF
	bl	_println
	ldp	x29, x30, [sp], #16             ; 16-byte Folded Reload
	ret
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc
                                        ; -- End function
	.section	__TEXT,__cstring,cstring_literals
l_str_pointer:                          ; @str_pointer
	.asciz	"%s%c"

l_str_pointer.1:                        ; @str_pointer.1
	.asciz	"Welkom my friend"

l_str_pointer.2:                        ; @str_pointer.2
	.asciz	"oaky %d %c"

l_str_pointer.3:                        ; @str_pointer.3
	.asciz	"limit reached."

l_str_pointer.4:                        ; @str_pointer.4
	.asciz	"this is kinda stupid.."

.subsections_via_symbols
