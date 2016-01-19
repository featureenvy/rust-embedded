mod device;
mod uart_register;

pub use self::device::Device;

pub enum Uarts {
    Uart0,
}
