#![allow(dead_code)]
use std::path::{Path, PathBuf};

const INPUT_FILES_LOCATION: &str = "tests/input";

pub fn crate_input_file_path(file_name: impl AsRef<Path>) -> PathBuf {
    input_file_path(PathBuf::from(env!("CARGO_CRATE_NAME")).join(file_name))
}

pub fn input_file_path(file_name: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(INPUT_FILES_LOCATION).join(file_name)
}
