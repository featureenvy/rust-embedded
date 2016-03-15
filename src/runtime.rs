use core;

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="panic_fmt"]
pub extern fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop { }
}

#[lang="begin_unwind"]
extern fn begin_unwind() {}

pub use core::intrinsics::breakpoint;

// Call the debugger and halts execution.
#[no_mangle]
pub extern fn abort() -> ! {
  unsafe {
    breakpoint();
  }
  loop {}
}
