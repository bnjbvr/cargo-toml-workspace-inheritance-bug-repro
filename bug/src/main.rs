fn main() {
    #[derive(serde::Deserialize)]
    struct Metadata {}

    if let Err(err) = cargo_toml::Manifest::<Metadata>::from_path_with_metadata("../foo/Cargo.toml") {
        eprintln!("error when reading the manifest: {}", err);
    } else {
        eprintln!("allright!");
    }
}
