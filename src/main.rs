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
use raylib::ffi::TraceLogLevel;


const TRANSPARENT_COLOR: Color = Color::new(0, 0, 0, 0);


fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    if !enemy.is_alive {
        return;
    }

    // Calcular ángulo del sprite relativo al jugador
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);
    let mut angle_diff = sprite_a - player.a;
    
    // Normalizar el ángulo
    while angle_diff > PI {
        angle_diff -= 2.0 * PI;
    }
    while angle_diff < -PI {
        angle_diff += 2.0 * PI;
    }

    // Verificar si el sprite está dentro del campo de visión
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    // Calcular distancia al sprite
    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    // Culling por distancia (plano cercano y lejano)
    if sprite_d < 30.0 || sprite_d > 600.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    // Calcular tamaño del sprite en pantalla
    let sprite_size = (screen_height / sprite_d) * 70.0;
    
    // Calcular posición X en pantalla
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    // Calcular bounds del sprite
    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as i32;
    let end_x = (screen_x + sprite_size / 2.0).min(screen_width) as i32;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as i32;
    let end_y = (screen_height / 2.0 + sprite_size / 2.0).min(screen_height) as i32;

    // Obtener coordenadas del frame actual
    let (frame_x, frame_y, frame_width, frame_height) = enemy.get_frame_coords(128 * enemy.frame_count as u32);

    // Dibujar el sprite
    for x in start_x..end_x {
        for y in start_y..end_y {
            // Mapear coordenadas de pantalla a coordenadas de textura
            let tex_x = ((x - start_x) as f32 / (end_x - start_x) as f32) * frame_width as f32;
            let tex_y = ((y - start_y) as f32 / (end_y - start_y) as f32) * frame_height as f32;
            
            let final_tx = (frame_x as f32 + tex_x) as u32;
            let final_ty = (frame_y as f32 + tex_y) as u32;

            let color = texture_manager.get_pixel_color(enemy.texture_key, final_tx, final_ty);
            
            // Solo dibujar píxeles no transparentes
            if color.a > 0 && color != TRANSPARENT_COLOR {
                framebuffer.set_pixel_fast(x as u32, y as u32, color);
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
    let px = (player.pos.x / block_size as f32 * block_size as f32) as u32;
    let py = (player.pos.y / block_size as f32 * block_size as f32) as u32;
    framebuffer.set_pixel(px, py);

    for x in px.saturating_sub(1)..=px.saturating_add(1) {
        for y in py.saturating_sub(1)..=py.saturating_add(1) {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }

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
    enemies: &[Enemy],
    texture_cache: &TextureManager,
) {
    for enemy in enemies {
        draw_sprite(framebuffer, &player, &enemy, texture_cache);
    }
}

fn main() {
    let window_width = 1000;
    let window_height =800;
    let block_size = 80;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Project")
        .vsync()
        .build();

    window.set_target_fps(60);

    let internal_width = 500;  
    let internal_height = 400;


    //Load Music once before the loop
    let mut audio = RaylibAudio::init_audio_device();
    let music_forest = Sound::load_sound("musicforest.mp3") .expect("No se pudo cargar la música");
    
    audio.play_sound(&music_forest);
    audio.set_sound_volume(&music_forest, 0.5);

    let background_color = Color::BLACK;
    let mut framebuffer = Framebuffer::new(internal_width as u32, internal_height as u32, background_color);

    // Framebuffer para el mapa
    let mut fb_map = Framebuffer::new(130, 90, background_color);
    let map_block_size = 10; // Tamaño más pequeño para el mapa

    // Load the maze once before the loop
    let maze = load_maze("prueba.txt");

    //Load player
    let mut player = Player{pos: Vector2::new(150.0,150.0), a: PI/3.0, fov:PI/3.0};

    //Load textures
    let texture_cache = TextureManager::new(&mut window, &raylib_thread);

    //Crear enemigos
    let mut enemies = vec![
        Enemy::new(500.0, 100.0, 'e', 4), // Enemigo animado con 4 frames
    ];

    let sky_color = Color::new(15, 4, 36, 255);
    let floor_color = Color::new(48, 35, 61, 255);

    let mut frame_count = 0;
    let mut last_time = std::time::Instant::now();
    let mut last_frame_time = std::time::Instant::now();

    while !window.window_should_close() {

        // Calcular delta time
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        // Actualizar enemigos
        for enemy in &mut enemies {
            enemy.update(delta_time);
        }
        
        // Clear framebuffer
        framebuffer.clear();
        fb_map.clear();

        //Procesar eventos
        process_events(&window, &mut player, block_size, &maze);

        // Renderizar el modo 3D
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

        render_3D(&mut framebuffer, &maze, block_size, &player, &texture_cache );

        // Renderizar enemigos
        render_enemies(&mut framebuffer, &player, &enemies, &texture_cache);
        
        // Renderizar mapa
        render_maze(&mut fb_map, &maze, map_block_size, &player);

        // Dibujar al jugador en el mini mapa
        let player_map_x = (player.pos.x / block_size as f32 * map_block_size as f32) as u32;
        let player_map_y = (player.pos.y / block_size as f32 * map_block_size as f32) as u32;

        fb_map.set_current_color(Color::VIOLET);
        // Dibujar una cruz para mayor visibilidad
        for offset in -1..=1 {
            fb_map.set_pixel(player_map_x.saturating_add_signed(offset), player_map_y);
            fb_map.set_pixel(player_map_x, player_map_y.saturating_add_signed(offset));
        }

        // Swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        {
            //Draw fps
            let mut d = window.begin_drawing(&raylib_thread);
            d.draw_fps(10, 10);

            let map_scale = 1.0;
            let map_display_width = (fb_map.width as f32 * map_scale) as i32;
            let map_display_height = (fb_map.height as f32 * map_scale) as i32;
            let map_x = window_width - map_display_width - 20;
            let map_y = 20;

            d.draw_rectangle(
                map_x - 5, map_y - 5, 
                map_display_width + 10, map_display_height + 10,
                Color::new(0, 0, 0, 180) // Negro semitransparente
            );

            // Dibujar el mapa
            for y in 0..fb_map.height {
                for x in 0..fb_map.width {
                    let index = (y * fb_map.width + x) as usize;
                    if index < fb_map.color_buffer.len() {
                        let color = fb_map.color_buffer[index];
                        if color != fb_map.background_color {
                            d.draw_rectangle(
                                map_x + x as i32,
                                map_y + y as i32,
                                1, 1, // Dibujar pixel por pixel
                                color
                            );
                        }
                    }
                }
            }
        }

        frame_count += 1;
        if last_time.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = std::time::Instant::now();
        }

    }
}
