#![no_std]

extern crate alloc;

mod util;
pub use util::*;

use kos_memory::talc;


static mut ARENA: [u8; 32768] = [0; 32768];

#[global_allocator]
static ALLOCATOR: talc::Talck<spin::Mutex<()>, talc::ClaimOnOom> = talc::Talc::new(unsafe {
    // if we're in a hosted environment, the Rust runtime may allocate before
    // main() is called, so we need to initialize the arena automatically
    talc::ClaimOnOom::new(talc::Span::from_const_array(core::ptr::addr_of!(ARENA)))
}).lock();