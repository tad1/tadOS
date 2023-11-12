ENTRY(Init);
EXTERN(Loop);

MEMORY{
    RAM : ORIGIN = 0x0, LENGTH = 16M
}

SECTIONS
{
    . = 0x80000;
    .text : {
        *(.text .text.*)
        } > RAM

}