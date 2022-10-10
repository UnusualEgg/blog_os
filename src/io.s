inbyte:
    mov ax,si
    in al, dx
    ret
outbyte:
    mov ax,si
    mov dx,di
    out dx, al
    ret