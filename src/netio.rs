use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

use crate::dictionary::Translations;
use crate::game::IO;

pub struct NetIO {}

struct NetClientIO {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl NetClientIO {
    fn new(stream: TcpStream) -> NetClientIO {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream);
        NetClientIO { reader, writer }
    }
}

impl IO for NetClientIO {
    fn read(&mut self) -> Result<String> {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        Ok(input)
    }

    fn write(&mut self, text: &str) -> Result<()> {
        self.writer.write_all(text.as_bytes()).unwrap();
        self.writer.flush().unwrap();
        Ok(())
    }
}

impl NetIO {
    pub fn run<F>(translations: Arc<Translations>, address: &str, callback: F)
    where
        F: Fn(Box<dyn IO>, Arc<Translations>) + Sync + Send + Copy + 'static,
    {
        let listener = TcpListener::bind(&address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let translations = translations.clone();
            std::thread::spawn(move || {
                let io = Box::new(NetClientIO::new(stream));
                callback(io, translations);
            });
        }
    }
}
