use core;

pub fn set_value(pointer: *mut u32, value: u32) {
    unsafe {
      let current_value = core::intrinsics::volatile_load(pointer);
      let next_value = current_value | value;
      core::intrinsics::volatile_store(pointer, next_value);
  }
}

pub fn set(pointer: *mut u32, bit: u32) {
    let value = 0x1 << bit;
    set_value(pointer, value);
}

pub fn clear_value(pointer: *mut u32, value: u32) {
  unsafe {
      let current_value = core::intrinsics::volatile_load(pointer);
      let next_value = current_value & !value;
      core::intrinsics::volatile_store(pointer, next_value);
  }
}

pub fn clear(pointer: *mut u32, bit: u32) {
    let value = 0x1 << bit;
    clear_value(pointer, value);
}

pub fn read_value(pointer: *mut u32, mask: u32) -> u32 {
    unsafe {
      let value = core::intrinsics::volatile_load(pointer);
      return value & mask;
  }
}

pub fn read(pointer: *mut u32, bit: u32) -> u32 {
    let mask = 0x1 << bit;
    return read_value(pointer, mask);
}

pub fn write(pointer: *mut u32, value: u32) {
  unsafe {
      core::intrinsics::volatile_store(pointer, value);
  }
}
