

.macro ADR_REL register, symbol
    adrp \register, \symbol
    add \register, \register, #:lo12:\symbol
.endm

.macro ADR_ABS register, symbol
	movz	\register, #:abs_g2:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
.endm

.section .text._start

_start:
    mrs	x0, MPIDR_EL1
    and x0, x0, {CONST_CORE_ID_MASK}
    ldr x1, BOOT_CORE_ID
    cmp x0, x1
    b.ne .L_parking_loop

    ADR_ABS x0, __bss_start
    ADR_ABS x1, __bss_end_exclusive

.L_bss_init_loop:
    cmp x0, x1
    b.eq .L_relocate_binary
    stp xzr, xzr, [x0], #16
    b .L_bss_init_loop

.L_relocate_binary:
    ADR_REL x0, __binary_nonzero_start // the address binary got loaded to
    ADR_ABS x1, __binary_nonzero_start // the address binary was loaded to
    ADR_ABS x2, __binary_nonzero_end_exclusive

.L_copy_loop:
    ldr x3, [x0], #8
    str x3, [x1], #8
    cmp x1, x2
    b.lo .L_copy_loop

    // set stack pointer
    ADR_ABS x0, __boot_core_stack_end_exclusive
    mov sp, x0

    ADR_ABS x1, _start_rust
    br x1

.L_parking_loop:
    wfe
    b .L_parking_loop

.size _start, . -_start
.type _start function
.global _start