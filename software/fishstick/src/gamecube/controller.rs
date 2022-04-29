use crate::joybus::Joybus;

pub struct Controller<J: Joybus> {
    joybus: J,
}

impl<J: Joybus> Controller<J> {
    pub fn new(joybus: J) -> Self {
        Self { joybus }
    }

    pub fn calibrate(&self) -> Result<(), ()> {
        let send_packet = &[1, 2, 3];
        let expected_receive = [4, 5, 6];

        let receive_packet = self.joybus.send_receive(send_packet)?;

        match receive_packet == expected_receive {
            true => Ok(()),
            false => Err(())
        }
    }

    pub fn exists(&self) -> Result<bool, ()> {
        todo!()
    }

    pub fn status(&self) -> Result<&[i32; 69], ()> {
        todo!()
    }
}
