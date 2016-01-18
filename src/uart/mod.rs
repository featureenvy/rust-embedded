mod uart;
mod device;
mod uart_register;

pub use self::uart::Uart;

pub enum Uarts {
    Uart0,
}
