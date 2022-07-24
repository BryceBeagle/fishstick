use bitvec::prelude::*;
use bitvec::BitArr;

pub const JOYBUS_SIZE: usize = 128;

pub type Packet = BitArr!(for JOYBUS_SIZE, in u8, Msb0);

pub trait Joybus {
    fn send(&self, packet: &Packet) -> Result<(), ()>;
    fn send_receive(&self, packet: &Packet) -> Result<&Packet, ()>;
    fn send_receive_no_alloc(&self, packet: &Packet, receive_packet: &mut Packet) -> Result<(), ()>;
}
