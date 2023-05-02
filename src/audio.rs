use audio::SoundSource;
use ggez::{audio, Context};
use specs::{World, WorldExt};
use std::collections::HashMap;

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, audio::Source>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &String) {
        let _ = self
            .sounds
            .get_mut(sound)
            .expect("expected sound")
            .play_detached(context);
    }
}

pub fn initialize_sounds()
