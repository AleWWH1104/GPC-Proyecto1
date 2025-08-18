use raylib::prelude::*;
use crate::textures::TextureManager;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteType {
    creature,  
    prize,
}
pub struct Sprite {
    pub pos: Vector2,
    pub texture_key: char,
    pub current_frame: usize,
    pub frame_count: usize,
    pub frame_delay: f32,
    pub frame_time: f32,
    pub is_alive: bool,
    pub sprite_type: SpriteType,
    pub sound_key: &'static str
}

impl Sprite {
    pub fn new(x: f32, y: f32, texture_key: char, frame_count: usize, sprite_type: SpriteType) -> Self {
        //Sonidos para cada sprite
        let sound_key = match sprite_type {
            SpriteType::creature => "creature_whisper",
            SpriteType::prize => "shimmering",
        };

        Sprite {
            pos: Vector2::new(x, y),
            texture_key,
            current_frame: 0,
            frame_count,
            frame_delay: 0.2, // Segundos entre frames (más lento para mejor visualización)
            frame_time: 0.0,
            is_alive: true,
            sprite_type,
            sound_key
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_alive {
            return;
        }

        self.frame_time += delta_time;
        if self.frame_time >= self.frame_delay {
            self.frame_time = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frame_count;
        }
    }

    // Método para obtener las coordenadas de textura del frame actual
    pub fn get_frame_coords(&self, texture_width: u32) -> (u32, u32, u32, u32) {
        let frame_width = texture_width / self.frame_count as u32;
        let src_x = self.current_frame as u32 * frame_width;
        (src_x, 0, frame_width, 128) // Asumiendo altura de 128 píxeles
    }
}