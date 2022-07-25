use bitvec::prelude::*;
use crate::joybus::{Joybus, Packet};

// TODO: fill packet constants with expected controller data
const STATUS_PACKET: Packet = BitArray::ZERO;

pub struct Controller<J: Joybus> {
    joybus: J,
}

impl<J: Joybus> Controller<J> {
    pub fn new(joybus: J) -> Self {
        Self { joybus }
    }

    pub fn exists(&self) -> Result<bool, &'static str> {
        todo!()
    }

    pub fn calibrate(&self) -> Result<(), &'static str> {
        todo!()
    }

    pub fn status(&self) -> Result<Packet, &'static str> {
        match self.joybus.send_receive(&STATUS_PACKET) {
            Ok(packet) => Ok(packet),
            Err(message) => Err(message),
        }
    }
}
