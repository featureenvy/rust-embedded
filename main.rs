#![feature(no_std)]
#![feature(core)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

extern crate core;

#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop { }
}

const GPIO_PORTF_CR_R: *mut u32 = 0x40025524 as *mut u32;
const GPIO_PORTF_AMSEL_R: *mut u32 = 0x40025528 as *mut u32;
const GPIO_PORTF_DIR_R: *mut u32 = 0x40025400 as *mut u32;
const GPIO_PORTF_AFSEL_R: *mut u32 = 0x40025420 as *mut u32;
const GPIO_PORTF_PUR_R: *mut u32 = 0x40025510 as *mut u32;
const GPIO_PORTF_DEN_R: *mut u32 = 0x4002551C as *mut u32;
const GPIO_PORTF_DATA_R: *mut u32 = 0x400253FC as *mut u32;
const GPIO_PORTF_PCTL_R: *mut u32 = 0x4002552C as *mut u32;
const GPIO_RCGC_GPIO_R: *mut u32 = 0x400FE608 as *mut u32;

const SYSCTL_RCGC2_R: *mut u32 = 0x400FE108 as *mut u32;
const SYSCTL_RCC_R: *mut u32 = 0x400FE060 as *mut u32;
const SYSCTL_RCC2_R: *mut u32 = 0x400FE070 as *mut u32;
const SYSCTL_RIS_R: *mut u32 = 0x400FE050 as *mut u32;
const SYSTICK_STCTRL_R: *mut u32 = 0xE000E010 as *mut u32;
const SYSTICK_STRELOAD_R: *mut u32 = 0xE000E014 as *mut u32;
const SYSTICK_CURRENT_R: *mut u32 = 0xE000E018 as *mut u32;


fn set(pointer: *mut u32, value: u32) {
  unsafe {
      let current_value = core::intrinsics::volatile_load(pointer);
      let next_value = current_value | value;
      core::intrinsics::volatile_store(pointer, next_value);
  }
}

fn unset(pointer: *mut u32, value: u32) {
  unsafe {
      let current_value = core::intrinsics::volatile_load(pointer);
      let next_value = current_value & !value;
      core::intrinsics::volatile_store(pointer, next_value);
  }
}

fn read(pointer: *mut u32, mask: u32) -> u32 {
  unsafe {
      let value = core::intrinsics::volatile_load(pointer);
      return value & mask;
  }
}

fn write(pointer: *mut u32, value: u32) {
  unsafe {
      core::intrinsics::volatile_store(pointer, value);
  }
}


fn clock_init() {
    set(SYSCTL_RCC_R, 0x00000800); // enable bypass
    unset(SYSCTL_RCC_R, 0x00400000); // disable sysdiv
    unset(SYSCTL_RCC_R, 0x00000001); // enable main OSC
    while read(SYSCTL_RIS_R, 0x00000100) == 0 {}; // wait for main OSC to start
    unset(SYSCTL_RCC_R, 0x000007C0); // clear XTAL fields
    set(SYSCTL_RCC_R, 0x00000540); // select 16MHz XTAL
    unset(SYSCTL_RCC_R, 0x00000030); // select main oscillator
    unset(SYSCTL_RCC_R, 0x00002000); // power up PLL


    unset(SYSCTL_RCC_R, 0x07800000); // clear sysdiv
    set(SYSCTL_RCC_R, 0x01000000); // set div to 4 (50MHz)
    set(SYSCTL_RCC_R, 0x00400000); // enable sysdiv
    while read(SYSCTL_RIS_R, 0x00000040) == 0 {}; // wait for PLL
    unset(SYSCTL_RCC_R, 0x00000800);
    write(SYSCTL_RCC2_R, 0xc1004000);

    set(SYSCTL_RCGC2_R, 0x00000020);
    read(SYSCTL_RCGC2_R, 0x1);
}

fn led_init() {
    set(GPIO_RCGC_GPIO_R, 0x20);
    write(GPIO_PORTF_CR_R, 0x1F);
    write(GPIO_PORTF_AMSEL_R, 0x00);
    write(GPIO_PORTF_PCTL_R, 0x00000000);
    write(GPIO_PORTF_DIR_R, 0x0E);
    write(GPIO_PORTF_AFSEL_R, 0x00);
    write(GPIO_PORTF_PUR_R, 0x11);
    write(GPIO_PORTF_DEN_R, 0x1F);

    set(GPIO_PORTF_DATA_R, 0x2);
}

fn systick_init() {
    set(SYSTICK_STCTRL_R, 0x00000004); // set clk source to main osc
    set(SYSTICK_STCTRL_R, 0x00000001); // enable multi-shot

    set(SYSTICK_STRELOAD_R, 0x00FFFFFF); // 50'000'000
    read(SYSTICK_CURRENT_R, 0x1); // clear current bit

    set(SYSTICK_STCTRL_R, 0x00000002); // enable interrupts
}

#[no_mangle]
pub fn systick_handler() {
    let mut led = read(GPIO_PORTF_DATA_R, 0x2);
    led = led ^ 0x02;
    write(GPIO_PORTF_DATA_R, led);
}

#[no_mangle]
pub fn main() {
    clock_init();
    led_init();
    systick_init();

    loop {}
}
