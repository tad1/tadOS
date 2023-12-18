    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
1:
    wfi
    b       1b
2:
    mov     x1, #0x100000
    mov     sp, x1
