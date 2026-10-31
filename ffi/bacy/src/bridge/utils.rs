#[allow(
    clippy::unnecessary_box_returns,
    reason = "Diplomat requires boxed ownership for opaque ABI results"
)]
#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;
    use std::path::Path;

    use bacy::utils::crc_manipulator::CrcManipulator;
    use bacy::utils::strategy;

    use crate::error::error;
    use crate::error::ffi::BacyError;

    #[diplomat::opaque]
    pub struct BacyCrcManipulator(pub CrcManipulator);

    #[diplomat::opaque]
    pub struct BacyStrategy;

    impl BacyCrcManipulator {
        pub fn new(path: &str) -> Box<Self> { Box::new(Self(CrcManipulator::new(path))) }

        pub fn forge_crc(&self, target_crc: u32) -> Result<(), Box<BacyError>> {
            self.0.forge_crc(target_crc).map_err(error)
        }

        pub fn match_file(&self, target_path: &str) -> Result<(), Box<BacyError>> {
            self.0.match_file(Path::new(target_path)).map_err(error)
        }
    }

    impl BacyStrategy {
        pub fn get_file_path(
            path: &str,
            crc: Option<i64>,
            no_hash: bool,
            to_lower: bool,
            output: &mut DiplomatWrite
        ) -> Result<(), Box<BacyError>> {
            let path = strategy::get_file_path(path, crc, no_hash, to_lower);

            output.write_str(&path.to_string_lossy()).map_err(error)
        }
    }
}
