ENTRY(Loop);

SECTIONS
{
    . = 0x80000;
    .text : {*(.text .text.*)}

}