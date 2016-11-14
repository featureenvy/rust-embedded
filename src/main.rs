#![feature(lang_items)]
#![feature(core_intrinsics)]

#![no_main]
#![no_std]

pub mod runtime;
mod memory;
pub mod hal;
pub mod driver;
mod debug;
pub mod systick;
mod labs;

use labs::*;

#[no_mangle]
pub fn start() -> ! {
    lab4::run();
    // lab5::run();
    // lab6::run();
    // lab7::run();
    // lab8::run();
    // lab9::run();
    // lab10::run();

    loop { };
}

pub mod vector_table {
    #[link_section = ".reset"]
    pub static RESET: fn() -> ! = ::start;
}

