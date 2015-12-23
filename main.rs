#![feature(no_std)]
#![feature(core)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

mod runtime;
mod memory;
mod clock;
mod gpio;
mod led;
mod switch;
pub mod systick;

mod labs;

#[no_mangle]
pub fn main() {
    labs::lab1::run();
}
