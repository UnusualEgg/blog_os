section .text
global inbyte
inbyte:
    mov dx,si
    in al, dx
    ret