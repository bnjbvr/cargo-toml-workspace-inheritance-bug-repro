fn main() {
    if let Err(err) = cargo_toml::Manifest::from_path("./foo/Cargo.toml") {
        eprintln!("error when reading the manifest: {}", err);
    } else {
        eprintln!("allright!");
    }
}
