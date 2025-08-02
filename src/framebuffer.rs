use raylib::prelude::*;
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    
    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let screen_width = window.get_screen_width() as f32;
            let screen_height = window.get_screen_height() as f32;
    
            let mut rendering = window.begin_drawing(raylib_thread);
            rendering.clear_background(self.background_color);


            let scale_x = screen_width / self.width as f32;
            let scale_y = screen_height / self.height as f32;
            let scale = f32::min(scale_x, scale_y);

            let scaled_width = self.width as f32 * scale;
            let scaled_height = self.height as f32 * scale;

            let offset_x = (screen_width - scaled_width) / 2.0;
            let offset_y = (screen_height - scaled_height) / 2.0;

            rendering.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32), 
                Rectangle::new(offset_x, offset_y, scaled_width, scaled_height),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }
    
    pub fn get_color(&mut self, x: u32, y: u32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.get_color(x as i32, y as i32)
        } else {
            Color::BLACK
        }
    }
}