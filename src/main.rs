#![feature(start)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(fixed_size_array)]

//#[cfg(target_os = "none")]
extern crate rlibc;

mod runtime;
mod memory;
mod hal;
mod driver;
pub mod systick;

pub mod labs;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    //labs::lab1::run();
    labs::lab2::run();

    0
}
