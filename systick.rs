use memory::{set_value,read};


const SYSTICK_STCTRL_R: *mut u32 = 0xE000E010 as *mut u32;
const SYSTICK_STRELOAD_R: *mut u32 = 0xE000E014 as *mut u32;
const SYSTICK_CURRENT_R: *mut u32 = 0xE000E018 as *mut u32;

static mut handler: fn() = empty_handler;

pub fn init(systick_fn: fn()) {
    set_value(SYSTICK_STCTRL_R, 0x00000004); // set clk source to main osc
    set_value(SYSTICK_STCTRL_R, 0x00000001); // enable multi-shot

    set_value(SYSTICK_STRELOAD_R, 0x00FFFFFF); // 50'000'000
    read(SYSTICK_CURRENT_R, 0x1); // clear current bit

    set_value(SYSTICK_STCTRL_R, 0x00000002); // enable interrupts

    unsafe {
        handler = systick_fn;
    }
}

fn empty_handler() {}

#[no_mangle]
pub fn systick_handler() {
    unsafe {
        handler();
    }
}
