
#[cfg(any(target_arch = "x86_64", target_arch = "i686"))]
pub fn halt() -> ! {
    use core::arch::asm;

    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}