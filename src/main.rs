#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod enemy;

use raylib::prelude::*;
use raylib::core::audio::{ Sound, RaylibAudio };
use std::thread;
use std::time::{Duration, Instant};
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use player::{Player,process_events};
use caster::{cast_ray, Intersect};
use std::f32::consts::PI;
use textures::TextureManager;
use enemy::Enemy;


const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // near plane           far plane
    if sprite_d < 50.0 || sprite_d > 800.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;
    let sprite_size_usize = sprite_size as usize;
    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, tx, ty);
            
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

fn draw_cell(
    framebuffer: &mut Framebuffer,
    xo: usize,
    yo: usize,
    block_size: usize,
    cell: char,
) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::new(48, 35, 61, 255));

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;
            
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    //draw player
    framebuffer.set_current_color(Color::WHITE);
    let px = player.pos.x as u32;
    let py = player.pos.y as u32;
    framebuffer.set_pixel(px, py);
    
    // draw what the player sees
    let num_rays = 3;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, block_size, a, true);
    }
}

pub fn render_3D(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
    texture_cache: &TextureManager
){
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 /2.0;

    let fov_half = player.fov / 2.0;
    let fov_step = player.fov / num_rays as f32;

    for i in 0..num_rays{
        let a = player.a - fov_half + (i as f32 * fov_step);
        let angle_diff = a - player.a;

        let intersect = cast_ray(framebuffer, maze, player, block_size, a, false);
        let d = intersect.distance;
        let c = intersect.impact;
        
        // Corregir distancia para evitar efecto "fish-eye"
        let correct_distance = d * angle_diff.cos();
        
        let stake_height = (hh / correct_distance) * 100.0;
        let half_stake_height = stake_height / 2.0;
        let stake_top = (hh - half_stake_height).max(0.0) as u32;
        let stake_bottom = (hh + half_stake_height).min(framebuffer.height as f32) as u32;

        let tx = intersect.tx as u32;
        let height_diff = stake_bottom - stake_top;

        // OPTIMIZACIÓN: Acceso directo al buffer
        if height_diff > 0 {
            let ty_step = 128.0 / height_diff as f32;
            let mut ty = 0.0;
            
            let base_idx = i as usize;
            
            for y in stake_top..stake_bottom {
                let color = texture_cache.get_pixel_color(c, tx, ty as u32);
                let idx = (y * framebuffer.width) as usize + base_idx;
                
                // Verificación de bounds una sola vez
                if idx < framebuffer.color_buffer.len() {
                    framebuffer.color_buffer[idx] = color;
                }
                
                ty += ty_step;
            }
        }
    }
}

fn render_enemies(
    framebuffer: &mut Framebuffer,
    player: &Player,
    texture_cache: &TextureManager,
) {
    let enemies = vec![
        Enemy::new(250.0, 250.0, 'x'),
    ];

    for enemy in enemies {
        draw_sprite(framebuffer, &player, &enemy, texture_cache);
    }
}

fn main() {
    let window_width = 900;
    let window_height =800;
    let block_size = 65;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Project")
        .vsync()
        .build();

    window.set_target_fps(60);

    let internal_width = 450;  
    let internal_height = 400;

    //Load Music once before the loop
    let mut audio = RaylibAudio::init_audio_device();
    let music_forest = Sound::load_sound("musicforest.mp3") .expect("No se pudo cargar la música");
    
    audio.play_sound(&music_forest);
    audio.set_sound_volume(&music_forest, 0.5);

    let background_color = Color::BLACK;
    let mut framebuffer = Framebuffer::new(internal_width as u32, internal_height as u32, background_color);

    framebuffer.set_background_color(background_color);

    // Load the maze once before the loop
    let maze = load_maze("prueba.txt");

    //Load player
    let mut player = Player{pos: Vector2::new(150.0,150.0), a: PI/3.0, fov:PI/3.0};

    //Load textures
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    let sky_color = Color::new(15, 4, 36, 255);
    let floor_color = Color::new(48, 35, 61, 255);

    let mut frame_count = 0;
    let mut last_time = std::time::Instant::now();

    while !window.window_should_close() {
        
        // Clear framebuffer
        framebuffer.clear();

        //Procesar eventos
        process_events(&window, &mut player, block_size, &maze);

        // Renderizar según el modo
        if window.is_key_down(KeyboardKey::KEY_M){
            render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            let half_height = internal_height / 2;
            let half_size = (half_height * internal_width) as usize;

            if half_size <= framebuffer.color_buffer.len() {
                //Cielo
                framebuffer.color_buffer[0..half_size].fill(sky_color);
                // Piso 
                if half_size < framebuffer.color_buffer.len() {
                    framebuffer.color_buffer[half_size..].fill(floor_color);
                }
            }

            //Renderizar
            render_3D(&mut framebuffer, &maze, block_size, &player, &texture_cache );
            //render_enemies(&mut framebuffer, &player, &texture_cache);
        }

        // Swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        

        frame_count += 1;
        if last_time.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = std::time::Instant::now();
        }

        {
            let mut d = window.begin_drawing(&raylib_thread);
            d.draw_fps(10, 10); // Posición en esquina superior izquierda
        }

    }
}
