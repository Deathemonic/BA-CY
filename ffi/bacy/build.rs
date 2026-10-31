use std::io;
use std::path::Path;

use diplomat_core::hir::DocsUrlGenerator;
use diplomat_tool::config::Config;
use diplomat_tool::r#gen as generate;

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=src");

    generate(
        Path::new("src/lib.rs"),
        "c",
        Path::new("bindings"),
        &DocsUrlGenerator::default(),
        Config::default(),
        true
    )
}
