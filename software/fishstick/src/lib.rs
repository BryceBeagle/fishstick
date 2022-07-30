#![cfg_attr(not(test), no_std)]

pub mod gamecube;
pub mod joybus;

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::vec;

    #[test]
    fn health_check() -> Result<(), ()> {
        let _ = vec![1, 2, 3];
        Ok(())
    }
}
