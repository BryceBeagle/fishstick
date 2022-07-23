pub trait Joybus {
    fn send(&self, packet: &[i32]) -> Result<(), ()>;
    fn send_receive<const S: usize>(&self, packet: &[i32]) -> Result<[i32; S], ()>;
}
