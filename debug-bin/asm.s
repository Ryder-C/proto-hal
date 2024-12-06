.section .text.configure_cordic,"ax",%progbits
	.globl	configure_cordic
	.p2align	2
	.type	configure_cordic,%function
	.code	16
	.thumb_func
configure_cordic:
	.fnstart
	.cfi_sections .debug_frame
	.cfi_startproc
	ldr r0, .LCPI0_0
	movs r1, #89
	str r1, [r0]
	bx lr
	.p2align	2
	.long	1073875968
