#![no_std]
#![feature(const_int_from_str)]

extern crate alloc;

#[cfg(feature = "4k_pages")]
pub const PAGE_SIZE: usize = 4096;

#[cfg(feature = "16k_pages")]
pub const PAGE_SIZE: usize = 16384;

pub const STACK_SIZE: usize = {
    // Check if a specific size was set environment-wide, else use our pick.
    if let Some(env) = option_env!("KOS_INIT_STACK_SIZE") {
        if let Ok(v) = usize::from_str_radix(env, 10) {
            if v < 4096 {
                panic!("KOS_INIT_STACK_SIZE must be at least 4KiB.");
            }

            v
        } else {
            panic!("KOS_INIT_STACK_SIZE must be an integer.");
        }
    } else {
        if cfg!(debug_assertions) {
            0x100000
        } else {
            0x10000
        }
    }
};

mod memmap;
pub use memmap::*;
pub use talc;