#[diplomat::bridge]
pub mod ffi {
    #[diplomat::opaque]
    pub struct BacyBytes(pub Vec<u8>);

    impl BacyBytes {
        #[allow(
            clippy::needless_lifetimes,
            clippy::elidable_lifetime_names,
            reason = "Diplomat requires an explicit lifetime for borrowed byte slice returns"
        )]
        pub fn as_slice<'a>(&'a self) -> &'a [u8] { &self.0 }
    }
}
