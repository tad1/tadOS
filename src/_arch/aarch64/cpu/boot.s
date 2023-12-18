

.macro ADR_REL register, symbol
    adrp \register, \symbol
    add \register, \register, #:lo12:\symbol
.endm

.section .text._start

_start:
    mrs     x0, CurrentEL
    and     x0, x0, #12 // clear reserved bits
    cmp     x0, #12
    b.eq	.L_EL3_to_EL2

.el2_entry:
    mrs	x0, CurrentEL
    cmp	x0, {CONST_CURRENTEL_EL2}

    mrs	x1, MPIDR_EL1
    and x1, x1, {CONST_CORE_ID_MASK}
    ldr x2, BOOT_CORE_ID
    cmp x1, x2
    b.ne .L_parking_loop

    ADR_REL x0, __bss_start
    ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
    cmp x0, x1
    b.eq .L_prepare_rust
    stp xzr, xzr, [x0], #16
    b .L_bss_init_loop


.L_prepare_rust:
    // set stack pointer
    ADR_REL x0, __boot_core_stack_end_exclusive
    mov sp, x0

    ADR_REL x1, ARCH_TIMER_COUNTER_FREQUENCY
    mrs     x2, CNTFRQ_EL0
    cmp     x2, xzr

    b.eq    .L_parking_loop
    str     w2, [x1]

    b       _start_rust

.L_EL3_to_EL2:
    // Initialize SCTLR_EL2 and HCR_EL2 to save values before entering EL2.
    MSR SCTLR_EL2, XZR
    MSR HCR_EL2, XZR
    // Determine the EL2 Execution state.
    MRS X0, SCR_EL3
    ORR X0, X0, #(1<<10) // RW EL2 Execution state is AArch64.
    ORR X0, X0, #(1<<0) // NS EL1 is Non-secure world.
    MSR SCR_EL3, x0
    MOV X0, #0b01001 // DAIF=0000
    MSR SPSR_EL3, X0 // M[4:0]=01001 EL2h must match SCR_EL3.RW
    // Determine EL2 entry.
    ADR X0, .el2_entry // el2_entry points to the first instruction of
    MSR ELR_EL3, X0 // EL2 code.
    ERET



.L_parking_loop:
    wfe
    b .L_parking_loop

.size _start, . - _start
.type _start, function
.global _start