use super::Port;

#[derive(Copy, Clone)]
pub struct PortData {
    pub base_address: u32,
    pub position: u32,
}

impl PortData {
    pub fn new(port: Port) -> PortData {
        match port {
            Port::PortA => PortData{
                base_address: 0x4000_4000,
                position: 0},
            Port::PortF => PortData{
                base_address: 0x4002_5000,
                position: 5
            },
        }
    }
}
