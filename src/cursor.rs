// #[link(name="cursor.o")]
use core::{arch::{asm, global_asm}, u8};

use blog_os::println;
global_asm!(
    include_str!("io.s")
);
extern "C" {
    // fn outb(port:u16,value:u16);
    fn inbyte(port:u16) -> u32;
    fn outbyte(port:u16,value:u32) -> u32;
}
pub fn enable_cursor() {
    unsafe {
        outbyte(0x3D4, 0x0A);
        let curstart = inbyte(0x3D5) & 0x1F; // get cursor scanline start
        println!("curstart{}",curstart);
        outbyte(0x3D4, 0x0A);
        outbyte(0x3D5, curstart | 0x20); // set enable bit
    }
}