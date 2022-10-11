extern crate x86_64;

pub fn enable_cursor() {
    use core::arch::asm;
    unsafe{
        // asm!("outb 0x3D4, 0x09");   // set maximum scan line register to 15
        // asm!("outb 0x3D5, 0x0F");

        // asm!("outb 0x3D4, 0x0B");   // set the cursor end line to 15
        // asm!("outb 0x3D5, 0x0F");

        // asm!("outb 0x3D4, 0x0A");   // set the cursor start line to 14 and enable cursor visibility
        // asm!("outb 0x3D5, 0x0E");
    }
}