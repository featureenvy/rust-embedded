#![feature(start)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

//#[cfg(target_os = "none")]
extern crate rlibc;

mod runtime;
mod memory;
mod clock;
mod gpio;
mod led;
mod switch;
pub mod systick;

pub mod labs;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    labs::lab1::run();

    0
}
