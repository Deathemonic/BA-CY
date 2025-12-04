fn main() {
    #[cfg(feature = "uniffi")]
    uniffi::generate_scaffolding("src/bacy.udl").unwrap();
}
