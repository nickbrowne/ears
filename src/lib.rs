// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

/*!
# ears

A simple library to play sounds and music in Rust, using OpenAL and libsndfile.

# Functionality

`ears` provides two ways to play audio files.

* `Sound`, which is for short lived audio samples, like sound effects.
* `Music`, which is for longer audio and streamed from the disk.

# Example

```no_run
extern crate ears;
use ears::{Sound, SoundError, AudioController};

fn main() -> Result<(), SoundError> {
    // Create a new Sound.
    let mut snd = Sound::new("path/to/my/sound.ogg")?;

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}

    Ok(())
}
```
*/

#![crate_name = "ears"]
//#![desc = "Easy Api in Rust for Sounds"]
//#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![allow(dead_code, unused_attributes)]
//#![feature(macro_rules)]
//#![feature(unsafe_destructor)]
#![allow(unused_imports)]
//#![allow(raw_pointer_derive)]
#![allow(unused_must_use)]
//#![allow(improper_ctypes)]

extern crate libc;
#[macro_use]
extern crate lazy_static;

// Reexport public API
pub use audio_controller::AudioController;
pub use audio_tags::{AudioTags, Tags};
pub use einit::{init, init_in};
pub use error::SoundError;
pub use internal::{cleanup, OpenAlContextError};
pub use music::Music;
pub use presets::ReverbPreset;
pub use record_context::RecordContext;
pub use recorder::Recorder;
pub use reverb_effect::ReverbEffect;
pub use sound::Sound;
pub use sound_data::SoundData;
pub use states::State;

// Hidden internal bindings
mod internal;
mod openal;
mod sndfile;

// The public ears API

mod audio_controller;
mod audio_tags;
#[path = "init.rs"]
mod einit;
mod error;
pub mod listener;
mod music;
mod presets;
mod record_context;
mod recorder;
mod reverb_effect;
mod sound;
mod sound_data;
mod states;
