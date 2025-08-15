// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use framebuffer::Framebuffer;
use maze::{Maze,load_maze};
use player::{Player,process_events};
use caster::{cast_ray, Intersect};
use std::f32::consts::PI;
use textures::TextureManager;

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

    framebuffer.set_current_color(Color::RED);

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
    framebuffer.set_current_color(Color::GREEN  );
    let px = player.pos.x as u32;
    let py = player.pos.y as u32;
    framebuffer.set_pixel(px, py);
    
    // draw what the player sees
    let num_rays = 5;
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
){
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 /2.0;

    framebuffer.set_current_color(Color::RED);

    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = (player.a - (player.fov / 2.0)) + (player.fov * current_ray) + player.fov.cos();
        let angle_diff = a - player.a;

        let intersect = cast_ray(framebuffer, &maze, &player, block_size, a, false);
        let d = intersect.distance;
        let c = intersect.impact;
        let correct_distance = d * angle_diff.cos() as f32;
        
        let stake_height = (hh / correct_distance) * 100.0;
        let half_stake_height = stake_height /2.0;
        let stake_top = (hh - half_stake_height) as usize;
        let stake_bottom = (hh + half_stake_height) as usize;

        for y in stake_top..stake_bottom {
            match c {
                '+' => framebuffer.set_current_color(Color::BLUEVIOLET), 
                '-' => framebuffer.set_current_color(Color::BLUE), 
                '|' => framebuffer.set_current_color(Color::PURPLE), 
                _ => framebuffer.set_current_color(Color::PINK), 
            }
            framebuffer.set_pixel(i, y as u32);
        }

    }
}
fn main() {
    let window_width = 1000;
    let window_height =700;
    let block_size = 80;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let background_color = Color::BLACK;
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32, background_color);

    framebuffer.set_background_color(background_color);

    // Load the maze once before the loop
    let maze = load_maze("maze.txt");
    let mut player = Player{pos: Vector2::new(150.0,150.0), a: PI/3.0, fov:PI/2.0};


    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();

        process_events(&window, &mut player, block_size, &maze);

        // 2. draw the maze, passing the maze and block size
        let mut mode = "3D";

        if window.is_key_down(KeyboardKey::KEY_M){
            mode ="2D";
        }

        if mode =="2D"{
            render_maze(&mut framebuffer, &maze, block_size, &mut player);
        } else {
            render_3D(&mut framebuffer, &maze, block_size, &mut player);
        }

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }

}