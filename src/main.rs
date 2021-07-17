use std::env;
use std::sync::Arc;

mod dictionary;
mod game;
mod netio;
mod stdio;

use dictionary::Translations;
use game::{Session, IO};
use netio::NetIO;
use stdio::StdInOut;

fn play(io: Box<dyn IO>, translations: Arc<Translations>) {
    let mut cnt = 1;
    let mut game = Session::new(io, translations);
    loop {
        game.ask();

        if cnt == 10 {
            game.show_statistic();
            cnt = 0;
        }
        cnt += 1;
    }
}

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "./dictionary/ru-pl.txt".to_owned());

    let address = env::args().nth(2);

    let dict = Arc::new(dictionary::read(&path).expect("can't read dictionary"));

    if let Some(address) = address {
        println!("Network mode:{}", address);
        NetIO::run(dict, &address, play);
    } else {
        StdInOut::run(dict, play);
    }
}
