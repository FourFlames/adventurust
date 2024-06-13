use std::fs::{self, create_dir};
use std::io::{self};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Files {
    config_root: PathBuf,
    config_files: Vec<PathBuf>,
    data_root: PathBuf,
    _data_files: Vec<PathBuf>,
}

impl Files {
    pub fn read_lines_of(&self, name: &str) -> io::Result<Vec<String>> {
        let config_file = self.config_files.iter().find_map( |p| {
            let mut pb = PathBuf::from(&self.config_root);
            pb.push(name);
            pb.set_extension("cfg");
            if p == &pb {
                Some(p)
            } else {
                None
            }
        });

        if let Some(path) = config_file {
            let content = fs::read_to_string(path)?;
            Ok(content.lines().map(String::from).collect())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Config file not found"))
        }
    }
}

pub fn initialize() -> io::Result<Files> {
    let mut files = Files {
        config_root: "./config/".into(),
        config_files: Vec::new(),
        data_root: "./data/".into(),
        _data_files: Vec::new(),
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
