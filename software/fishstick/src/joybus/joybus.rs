use bitvec::prelude::*;

pub const JOYBUS_SIZE: usize = 128;

pub type Packet = BitArr!(for JOYBUS_SIZE, in u8);

pub trait Joybus {
    fn send(&self, packet: &Packet) -> Result<(), &'static str>;
    fn send_receive_no_alloc(&self, packet: &Packet, receive_packet: &mut Packet) -> Result<(), &'static str>;

    /// Allocates a new Packet, and calls into no_alloc with it
    fn send_receive(&self, packet: &Packet) -> Result<Packet, &'static str> {
        let mut receive_packet: Packet = BitArray::ZERO;
        match self.send_receive_no_alloc(&packet, &mut receive_packet) {
            Ok(_) => Ok(receive_packet),
            Err(message) => Err(message),
        }
    }
}

struct JoybusMock {}

impl Joybus for JoybusMock {
    /// does nothing and returns Ok
    fn send(&self, _: &Packet) -> Result<(), &'static str> {
        // do nothing! data was sent!
        Ok(())
    }

    /// fills the `receive_packet` with one/true values
    fn send_receive_no_alloc(&self, _: &Packet, receive_packet: &mut Packet) -> Result<(), &'static str> {
        for item in receive_packet.iter_mut() {
            item.commit(true)
        }
        Ok(())
    }
}
