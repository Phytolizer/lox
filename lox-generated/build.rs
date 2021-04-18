use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    lalrpop::Configuration::new()
        .process_file(manifest_dir.join("src").join("lox.lalrpop"))
        .unwrap();
}
