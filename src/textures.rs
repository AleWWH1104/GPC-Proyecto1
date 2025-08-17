use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images: HashMap<char, Image>,       // Store images for pixel access
    textures: HashMap<char, Texture2D>, // Store GPU textures for rendering
    pub animations: HashMap<char, (Texture2D, Image, usize)> // (texture, image, frame_count)
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        // Map characters to texture file paths
        let texture_files = vec![
            ('+', "assets/runas.png"),
            ('-', "assets/flor.png"),
            ('|', "assets/hiedra.png"),
            ('g', "assets/gema.png"),
            ('#', "assets/hiedra.png"), 
        ];

        for (ch, path) in texture_files {
            let image = Image::load_image(path).expect(&format!("Failed to load image {}", path));
            let texture = rl.load_texture(thread, path).expect(&format!("Failed to load texture {}", path));
            images.insert(ch, image);
            textures.insert(ch, texture);
        }

        let mut animations = HashMap::new();
        
        // Cargar textura de enemigo animado
        if let Ok(enemy_image) = Image::load_image("assets/enemy.png") {
            if let Ok(enemy_texture) = rl.load_texture_from_image(thread, &enemy_image) {
                images.insert('e', enemy_image.clone()); // También guardamos en images para pixel access
                animations.insert('e', (enemy_texture, enemy_image, 4)); // 4 frames de animación
            }
        }

        TextureManager { images, textures, animations }
    }

    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Color {
        // Primero buscar en animaciones
        if let Some((_, image, _)) = self.animations.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            return get_pixel_color(image, x, y);
        }
        
        // Luego buscar en texturas estáticas
        if let Some(image) = self.images.get(&ch) {
            let x = tx.min(image.width as u32 - 1) as i32;
            let y = ty.min(image.height as u32 - 1) as i32;
            get_pixel_color(image, x, y)
        } else {
            Color::WHITE
        }
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        // Primero buscar en animaciones
        if let Some((texture, _, _)) = self.animations.get(&ch) {
            return Some(texture);
        }
        
        // Luego buscar en texturas estáticas
        self.textures.get(&ch)
    }

    pub fn get_animation_info(&self, ch: char) -> Option<(u32, usize)> {
        if let Some((_, image, frame_count)) = self.animations.get(&ch) {
            Some((image.width as u32, *frame_count))
        } else {
            None
        }
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