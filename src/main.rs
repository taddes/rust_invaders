use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("win", "win.wav");
    audio.add("startup", "startup.wav");
    audio.play("startup");

    // Cleanup - run off main - audio sys plays on seperate thread in parallel
    audio.wait();
    Ok(())
}
