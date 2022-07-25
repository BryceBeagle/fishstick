use bitvec::prelude::*;
use crate::joybus::{Joybus, Packet};

// TODO: fill packet constants with expected controller data
const CALIBRATE_PACKET: Packet = BitArray::ZERO;
const STATUS_PACKET: Packet = BitArray::ZERO;

pub struct Controller<J: Joybus> {
    joybus: J,
    state: Packet,
}

impl<J: Joybus> Controller<J> {
    pub fn new(joybus: J) -> Self {
        Self { joybus, state: BitArray::ZERO }
    }

    pub fn get_cached_state(&self) -> &Packet {
        &self.state
    }

    pub fn process_state(&mut self) -> Result<&Packet, &'static str> {
        match self.joybus.send_receive_no_alloc(&STATUS_PACKET, &mut self.state) {
            Ok(_) => Ok(&self.state),
            Err(message) => Err(message),
        }
    }

    pub fn calibrate(&mut self) -> Result<(), &'static str> {
        match self.joybus.send(&CALIBRATE_PACKET) {
            Ok(_) => Ok(()),
            Err(message) => Err(message),
        }
    }

    pub fn exists(&self) -> Result<bool, &'static str> {
        todo!()
    }
}
