use std::fs::{self, create_dir};
use std::path::{Path, PathBuf};

pub struct Files {
    config_root: PathBuf,
    data_root: PathBuf,
}

pub fn initialize() -> std::io::Result<Files> {
    let files = Files {
        config_root: "./config/".into(),
        data_root: "./data/".into(),
    };
    for directory in [files.config_root.clone(), files.data_root.clone()] {
        match create_dir(&directory) {
            Ok(_n) => println!("Created {:?}", directory),
            Err(_n) => println!("Failed to create {:?}", directory),
        }
    }
    Ok(files)
}
