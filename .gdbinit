define tr
target remote localhost:3333
end

define ld
file target/thumbv7em-none-eabi/debug/rust-embedded
end
