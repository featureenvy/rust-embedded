#![feature(start)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

//#[cfg(target_os = "none")]
extern crate rlibc;

pub mod runtime;
mod memory;
mod hal;
mod driver;
mod debug;
pub mod systick;

pub mod labs;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    labs::lab4::run();
    // labs::lab5::run();
    // labs::lab6::run();
    // labs::lab7::run();
    // labs::lab8::run();
    // labs::lab9::run();

    0
}
