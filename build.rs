extern crate gcc;

fn main() {
    gcc::Config::new().compiler("arm-none-eabi-gcc").file("src/LM4F_startup.c").flag("-mthumb").compile("libstartup.a");
}
