use raylib::audio::RaylibAudio;
use std::collections::HashMap;
use raylib::core::audio::{ Sound};
use raylib::prelude::*;
use crate::sprites::{self, Sprite};

pub struct AudioSystem<'a> {
    pub sounds: HashMap<&'static str, Sound>,
    pub audio: &'a mut RaylibAudio,
    pub background_playing: bool
}

impl<'a> AudioSystem<'a> {
    pub fn new(audio: &'a mut RaylibAudio) -> Self {
        AudioSystem {
            sounds: HashMap::new(),
            audio,
            background_playing: true,
        }
    }

    pub fn load_sound(&mut self, key: &'static str, path: &str) {
        let sound = Sound::load_sound(path).expect("Error al cargar sonido");
        self.sounds.insert(key, sound);
    }

    pub fn play_proximity_sounds(&mut self, player_pos: Vector2, sprites: &[Sprite]) {
        const PROXIMITY_RADIUS: f32 = 200.0;
        
        for sprite in sprites {
            // Correct distance calculation
            let dx = player_pos.x - sprite.pos.x;
            let dy = player_pos.y - sprite.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < PROXIMITY_RADIUS {
                let volume = 2.0 - (distance / PROXIMITY_RADIUS).powf(2.0);
                if let Some(sound) = self.sounds.get_mut(sprite.sound_key) {
                    self.audio.set_sound_volume(sound, volume);
                    
                    if !self.audio.is_sound_playing(sound) {
                        self.audio.play_sound(sound);
                    }
                }
            } else {
                if let Some(sound) = self.sounds.get(sprite.sound_key) {
                    self.audio.stop_sound(sound);
                }
            }
        }
    }

    pub fn update_zone_music(
        &mut self,
        in_special_zone: bool,
        background_music: &Sound,
        zone_music: &Sound,
    ) {
        match (self.background_playing, in_special_zone) {
            (true, true) => {
                self.audio.stop_sound(background_music);
                self.audio.play_sound(zone_music);
                self.background_playing = false;
            }
            (false, false) => {
                self.audio.stop_sound(zone_music);
                self.audio.play_sound(background_music);
                self.background_playing = true;
            }
            _ => {}
        }
        
        // Ensure proper volume when transitioning
        self.audio.set_sound_volume(background_music, 0.5);
        self.audio.set_sound_volume(zone_music, 0.5);
    }
}