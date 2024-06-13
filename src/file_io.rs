use std::fs::{self, create_dir};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Files {
    config_root: PathBuf,
    config_files: Vec<PathBuf>,
    data_root: PathBuf,
    data_files: Vec<PathBuf>,
}

pub fn initialize() -> std::io::Result<Files> {
    let mut files = Files {
        config_root: "./config/".into(),
        config_files: Vec::new(),
        data_root: "./data/".into(),
        data_files: Vec::new(),
    };
    for directory in
    [files.config_root.clone(), files.data_root.clone()] {
        match directory.try_exists()? {
            true => println!("Found {:?}", directory),
            false => {
                println!("Could not find {:?}", directory);
                match create_dir(&directory) {
                    Ok(_n) => println!("Created {:?}", directory),
                    Err(_n) => panic!("Failed to create {:?}", directory),
                }
            },
        }
        // Now we know for sure the directory exists.
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.extension().unwrap() == "cfg" {
                files.config_files.push(path);
            }
        }
    }
    println!("{:?}", files);
    Ok(files)
}
