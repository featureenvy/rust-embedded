use core;

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop { }
}

#[lang="begin_unwind"]
extern fn begin_unwind() {}