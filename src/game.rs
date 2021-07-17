use rand::Rng;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;

use crate::dictionary::Translations;

pub struct Statistic {
    correct: usize,
    incorrect: usize,
}

pub trait IO {
    fn read(&mut self) -> Result<String>;
    fn write(&mut self, text: &str) -> Result<()>;
    fn writeln(&mut self, text: &str) -> Result<()> {
        self.write(&format!("{}\r\n", text))
    }
}

impl Default for Statistic {
    fn default() -> Statistic {
        Statistic {
            correct: 0,
            incorrect: 0,
        }
    }
}

impl Statistic {
    fn update(&mut self, result: bool) {
        if result {
            self.correct += 1;
        } else {
            self.incorrect += 1;
        }
    }
}

pub struct Session {
    statistic: Statistic,
    io: Box<dyn IO>,
    translations: Arc<Translations>,
}

impl Session {
    pub fn new(io: Box<dyn IO>, translations: Arc<Translations>) -> Session {
        Session {
            statistic: Statistic::default(),
            io,
            translations,
        }
    }

    pub fn ask(&mut self) {
        const N: usize = 4;
        let max_idx = self.translations.len() - 1;
        let idx = rand::thread_rng().gen_range(0..N);

        let variants = [
            &self.translations[rand::thread_rng().gen_range(0..max_idx)],
            &self.translations[rand::thread_rng().gen_range(0..max_idx)],
            &self.translations[rand::thread_rng().gen_range(0..max_idx)],
            &self.translations[rand::thread_rng().gen_range(0..max_idx)],
        ];

        self.io.writeln(&variants[idx].native).unwrap();

        for (i, v) in variants.iter().enumerate() {
            self.io
                .writeln(&format!("    {}:{}", i + 1, v.foreign))
                .unwrap();
        }

        let input = self.io.read().unwrap();
        if let Ok(num) = read_num(&input) {
            let correct = num == idx + 1;
            if correct {
                self.io.writeln("Yes").unwrap();
            } else {
                self.io
                    .writeln(&format!(
                        "No. The valid answer is '{}'",
                        variants[idx].foreign
                    ))
                    .unwrap();
            }
            self.statistic.update(correct);
        } else {
            self.io.writeln("can't read input :(").unwrap();
        }
        self.io.writeln("").unwrap();
    }

    pub fn show_statistic(&mut self) {
        self.io
            .writeln("------------------------------------------------------------------------")
            .unwrap();
        self.io
            .writeln(&format!("Correct:{}", self.statistic.correct))
            .unwrap();
        self.io
            .writeln(&format!("Incorrect:{}", self.statistic.incorrect))
            .unwrap();
        self.io
            .writeln("------------------------------------------------------------------------")
            .unwrap();
    }
}

pub fn read_num(input: &str) -> Result<usize> {
    let input: String = input.chars().filter(|c| c.is_numeric()).collect();
    input
        .parse::<usize>()
        .map_err(|_| Error::new(ErrorKind::Other, ""))
}
