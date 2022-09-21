use std::{ffi::OsString, fs::read_dir};

use vergen::{vergen, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup version generation
    vergen(Config::default())?;
    // load protos from directory
    let paths: Vec<OsString> = read_dir("proto")
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().ends_with(".proto")
        })
        .map(|entry| entry.file_name())
        .collect();
    tonic_build::configure().compile(&paths, &["proto"])?;
    Ok(())
}
