cargo build --target thumbv7em-none-eabi --release
openocd -f oocd.cfg -c "program target/thumbv7em-none-eabi/release/rust-embedded verify reset exit"
