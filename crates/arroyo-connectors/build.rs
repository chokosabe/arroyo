fn main() -> Result<(), String> {
    // recursively find all json files in the src directory
    glob::glob("src/**/*.json")
        .unwrap()
        .filter_map(Result::ok)
        .for_each(|path| {
            println!("cargo:rerun-if-changed={}", path.display());
        });

    Ok(())
}
