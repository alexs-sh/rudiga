use std::sync::Arc;

use crate::dictionary::Translations;
use crate::game::IO;

use std::io::prelude::*;
use std::io::{stdin, stdout, Error, ErrorKind, Result};
pub struct StdInOut {}

impl IO for StdInOut {
    fn read(&mut self) -> Result<String> {
        let mut buffer = [0u8; 128];
        let len = stdin().read(&mut buffer)?;
        std::str::from_utf8(&buffer[..len])
            .map_or(Err(Error::new(ErrorKind::Other, "")), |v| Ok(v.to_owned()))
    }

    fn write(&mut self, text: &str) -> Result<()> {
        match stdout().write(text.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl StdInOut {
    pub fn run<F>(translations: Arc<Translations>, callback: F)
    where
        F: Fn(Box<dyn IO>, Arc<Translations>) + Sync + Send + Copy + 'static,
    {
        let io = StdInOut {};
        callback(Box::new(io), translations);
    }
}
