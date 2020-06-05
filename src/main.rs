use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::queue;
use crossterm::terminal::{self, ClearType};
use std::io::{self, BufWriter, Write};
use std::time::{Duration, Instant};

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = BufWriter::new(io::stdout());

    let mut old_tempo = None;
    let mut old_time = Instant::now();
    loop {
        queue!(stdout, terminal::Clear(ClearType::CurrentLine))?;

        match old_tempo {
            Some(tempo) => write!(stdout, "\r{}bpm: ", tempo),
            None => write!(stdout, "\rTap: "),
        }?;
        stdout.flush()?;

        match loop {
            let event = event::read()?;
            if let Event::Key(key) = event {
                break (key.code, key.modifiers);
            }
        } {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) | (KeyCode::Char('q'), ..) => break,
            _ => (),
        }

        let time = Instant::now();
        old_tempo = Duration::from_secs(60)
            .as_nanos()
            .checked_div((time - old_time).as_nanos());
        old_time = time;
    }
    print!("\n\r");
    terminal::disable_raw_mode()?;

    Ok(())
}
