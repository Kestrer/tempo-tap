use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::clear;
use std::time::{Instant, Duration};
use std::io::{self, ErrorKind, Write};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut keys = io::stdin().keys();

    let mut old_tempo = None;
    let mut old_time = Instant::now();
    loop {
        print!("{}\r", clear::CurrentLine);
        match old_tempo {
            Some(tempo) => print!("{}bpm: ", tempo),
            None => print!("Tap: "),
        };
        stdout.flush()?;

        match keys.next().ok_or(ErrorKind::UnexpectedEof)?? {
            Key::Ctrl('c') | Key::Char('q') => break,
            _ => {},
        };

        let time = Instant::now();
        old_tempo = Duration::from_secs(60).as_nanos().checked_div((time - old_time).as_nanos());
        old_time = time;
    }
    print!("\n\r");

    Ok(())
}
