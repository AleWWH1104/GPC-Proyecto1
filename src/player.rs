
use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, block_size: usize, maze: &Maze) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 20.0;

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED; 
    } 
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED; 
    } 

    let mut moved = false;
    let mut next_pos = player.pos;

    if window.is_key_down(KeyboardKey::KEY_UP) {
        next_pos.x += MOVE_SPEED * player.a.cos();
        next_pos.y += MOVE_SPEED * player.a.sin();
        moved = true;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        next_pos.x -= MOVE_SPEED * player.a.cos();
        next_pos.y -= MOVE_SPEED * player.a.sin();
        moved = true;
    }

    
    // Si el jugador intentó moverse, verificamos la nueva posición.
    if moved {
        let grid_x = next_pos.x as usize / block_size;
        let grid_y = next_pos.y as usize / block_size;

        // Verificamos si la nueva posición es un espacio vacío y está dentro de los límites.
        if grid_y < maze.len() && grid_x < maze[grid_y].len() && maze[grid_y][grid_x] == ' ' {
            // Si el camino está libre, actualizamos la posición del jugador.
            player.pos = next_pos;
        }
        // Si la condición no se cumple (es una pared), no hacemos nada.
        // El jugador simplemente no se moverá, quedando bloqueado por la pared.
    }
}