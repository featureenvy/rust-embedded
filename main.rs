#![feature(no_std)]
#![feature(core)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

mod runtime;
mod memory;
mod clock;
mod led;
pub mod systick;

#[no_mangle]
pub fn main() {
    clock::init();
    led::init();
    systick::init(led::toggle);

    loop {}
}
