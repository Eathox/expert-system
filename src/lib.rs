use std::{
    any::type_name,
    fmt,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    path::Path,
    result::Result,
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("file contains no data")]
    Empty,
    #[error("convert error <{0}>")]
    Parse(&'static str),
    #[error("read error")]
    Read { source: io::Error },
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub fn read_file<T: FromStr>(filename: impl AsRef<Path>) -> Result<Vec<T>, ReadFileError>
where
    T::Err: fmt::Debug,
{
    let file = File::open(filename).map_err(|source| ReadFileError::Read { source })?;
    let file_buf = BufReader::new(file);

    let mut result = vec![];
    for line in file_buf.lines() {
        result.push(
            line.map_err(|source| ReadFileError::Read { source })?
                .parse::<T>()
                .map_err(|_| ReadFileError::Parse(type_name::<T>()))?,
        )
    }

    if result.len() == 0 {
        Err(ReadFileError::Empty)?;
    }
    Ok(result)
}
