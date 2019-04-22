extern crate ears;

use ears::{Music, AudioController};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut music = Music::new("res/music.ogg").unwrap();
    music.play();
    music.set_looping(true);
    // music.set_offset(60.0);
    while music.is_playing() {
        println!("Offset: {:?}", music.get_offset());
        sleep(Duration::from_millis(1000));
    }
}
