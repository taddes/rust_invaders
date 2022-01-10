use std::error::Error;
use rusty_audio::Audio;
use std::io;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("win", "win.wav");
    audio.add("startup", "startup.wav");
    audio.play("startup");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
      while event::poll(Duration::default())? {
        if let Event::Key(key_event) = event::read()? {
          match key_event.code {
            KeyCode::Esc | KeyCode::Char('q) => {
              audio.play("lose");
              break 'gameloop
            }
          }
        }
      }
    }

    // Cleanup - run off main - audio sys plays on seperate thread in parallel
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
