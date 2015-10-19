rustc -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g --emit obj -L libcore-thumbv7em -o runtime.o runtime.rs

arm-none-eabi-ld -T LM4F.ld -o embedded.axf runtime.o
#openocd -f oocd.cfg -c "program embedded.axf verify reset" -c "shutdown"
