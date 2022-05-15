use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use ggrs::PlayerHandle;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct InputProtocol {
    /// This is the number of bytes one peer’s input is.
    /// In our case, the input consists of four direction buttons, and eventually the fire button as well.
    /// This means it fits easily within a single byte:
    pub input: u8,
}

impl InputProtocol {
    pub fn new(input: InputFlags) -> Self {
        InputProtocol {
            input: input.bits(),
        }
    }
}

bitflags! {
    pub struct InputFlags: u8 {
        const UP = 1 << 0;
        const DOWN = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
        const FIRE = 1 << 4;
    }
}

impl From<InputFlags> for InputProtocol {
    fn from(input: InputFlags) -> Self {
        Self::new(input)
    }
}

impl TryFrom<InputProtocol> for InputFlags {
    type Error = String;
    fn try_from(protocol: InputProtocol) -> Result<Self, Self::Error> {
        Self::from_bits(protocol.input).ok_or_else(|| {
            format!(
                "Failed to read protocol bits as valid inputs. Received: {}",
                protocol.input
            )
        })
    }
}

pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}
