#[diplomat::bridge]
pub mod ffi {
    use bacy::math::mersenne::MersenneTwister;

    use crate::error::ffi::BacyError;
    use crate::error::{Error, error};

    #[diplomat::opaque_mut]
    pub struct BacyMersenneTwister(pub MersenneTwister);

    impl BacyMersenneTwister {
        pub fn new(seed: u32) -> Box<Self> { Box::new(Self(MersenneTwister::new(seed))) }

        pub fn next_value(&mut self) -> u32 { self.0.next() }

        pub fn next_u32(&mut self) -> u32 { self.0.next_u32() }

        pub fn next_u64(&mut self) -> u64 { self.0.next_u64() }

        pub fn next_bytes(&mut self, output: &mut [u8]) { self.0.next_bytes(output); }

        pub fn next_f32(&mut self) -> f32 { self.0.next_f32() }

        pub fn next_f64(&mut self) -> f64 { self.0.next_f64() }

        pub fn next_range(&mut self, min: i32, max: i32) -> Result<i32, Box<BacyError>> {
            max.max(min)
                .checked_sub(max.min(min))
                .ok_or_else(|| error(Error::RandomRangeOverflow))?;

            Ok(self.0.next_range(min, max))
        }
    }
}
