use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

pub struct WordsPair {
    pub native: String,
    pub foreign: String,
}

pub type Translations = Vec<WordsPair>;

pub fn read(filename: &str) -> Result<Translations> {
    let mut result = Translations::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .filter(|s| !s.starts_with('#'))
        .collect();

    fn char_filter(c: &char) -> bool {
        c.is_alphabetic() || *c == '(' || *c == ')'
    }

    for line in lines {
        let words: Vec<&str> = line.split(' ').take(2).collect();

        if words.len() >= 2 {
            let native: String = words[0].chars().filter(char_filter).collect();
            let foreign: String = words[1].chars().filter(char_filter).collect();
            if words.len() >= 3 {
                let foreign1: String = words[2].chars().filter(char_filter).collect();
                if foreign1.len() > 2 {
                    result.push(WordsPair {
                        native,
                        foreign: format!("{} {}", foreign, foreign1),
                    });
                }
            } else {
                result.push(WordsPair { native, foreign });
            }
        }
    }
    Ok(result)
}
