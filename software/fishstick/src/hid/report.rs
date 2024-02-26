use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (usage_page = BUTTON, usage_min = 1, usage_max = 16) = {
            #[packed_bits 16] #[item_settings data,variable,absolute,not_null,no_wrap,linear] buttons = input;
        };
    }
)]
pub struct GamepadReport {
    pub buttons: [u8; 2],
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple test to make sure a GamepadReport can be constructed and a descriptor generated
    #[test]
    fn health_check() -> Result<(), ()> {
        let _ = GamepadReport {
            buttons: [0b0000_0000, 0b1111_1111],
        };

        let _ = GamepadReport::desc();

        Ok(())
    }
}
