use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

use crate::sprites::SpriteType;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
    pub animations: HashMap<SpriteType, (Texture2D, Image, usize)> // (texture, image, frame_count)
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();
        let mut animations = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('+', "assets/piedra.png"),
            ('-', "assets/flores.png"),
            ('|', "assets/flores.png"),
            ('l', "assets/1.png"),
            ('_', "assets/runas.png"),
            ('#', "assets/hiedra.png"), 
        ];

        for (ch, path) in texture_files {
            let image = Image::load_image(path).expect(&format!("Failed to load image {}", path));
            let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
            images.insert(ch, image);
            textures.insert(ch, texture);
        }

        let animation_files = vec![
            (SpriteType::creature, "assets/enemy.png", 4),
            (SpriteType::prize, "assets/prize.png", 1),
        ];

        for (sprite_type, path, frame_count) in animation_files {
            if let Ok(image) = Image::load_image(path) {
                if let Ok(texture) = rl.load_texture_from_image(thread, &image) {
                    animations.insert(sprite_type, (texture, image, frame_count));
                }
            }
        }

        TextureManager { images, textures, animations }
    }

    pub fn get_sprite_texture(&self, sprite_type: &SpriteType) -> Option<&Texture2D> {
        self.animations.get(sprite_type).map(|(texture, _, _)| texture)
    }

    pub fn get_sprite_animation_info(&self, sprite_type: &SpriteType) -> Option<(u32, usize)> {
        self.animations.get(sprite_type)
            .map(|(_, image, frame_count)| (image.width as u32, *frame_count))
    }

    pub fn get_sprite_pixel_color(&self, sprite_type: &SpriteType, tx: u32, ty: u32) -> Color {
        if let Some((_, image, _)) = self.animations.get(sprite_type) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            return get_pixel_color(image, x, y);
        }
        Color::WHITE
    }

    // Métodos para texturas estáticas (se mantienen igual)
    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}

fn get_pixel_color(image: &Image, x: i32, y: i32) -> Color {
    let width = image.width as usize;
    let height = image.height as usize;

    if x < 0 || y < 0 || x as usize >= width || y as usize >= height {
        return Color::new(255, 0, 255, 255); // Magenta para debug
    }

    let x = x as usize;
    let y = y as usize;

    let data_len = width * height * 4;

    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, data_len);

        let idx = (y * width + x) * 4;

        if idx + 3 >= data_len {
            return Color::new(255, 0, 255, 255); // Magenta para debug
        }

        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}