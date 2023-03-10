use std::path::Path;
use std::fs;

fn main() {
    let dest_path = Path::new("./target/debug/").join("install.toml");
    fs::write(
        &dest_path,
        "\
        pack = \"test.tests\"\r\n\
        name = \"install test\"\r\n\
        version = \"1.1\"\r\n\
        authors = \"test\"\r\n\
        description = \"Something\"\r\n\
        install_path = \"Here\"
        "
    ).expect("Some");
    println!("cargo:rerun-if-changed=build.rs");

}