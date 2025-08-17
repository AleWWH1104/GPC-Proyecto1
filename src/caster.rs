use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::player::{self, Player};
use crate::maze::Maze;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
    pub side: bool
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer, 
    maze: &Maze, 
    player: &Player, 
    block_size: usize, 
    a: f32, 
    draw: bool
) -> Intersect {
    let dx = a.cos();
    let dy = a.sin();
    
    // Posición actual en coordenadas del mapa
    let mut map_x = (player.pos.x / block_size as f32) as i32;
    let mut map_y = (player.pos.y / block_size as f32) as i32;
    
    // Distancia entre intersecciones consecutivas
    let delta_dist_x = if dx == 0.0 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy == 0.0 { 1e30 } else { (1.0 / dy).abs() };
    
    // Calcular step y side_dist inicial
    let (mut side_dist_x, step_x) = if dx < 0.0 {
        let step_x = -1;
        let side_dist_x = (player.pos.x / block_size as f32 - map_x as f32) * delta_dist_x;
        (side_dist_x, step_x)
    } else {
        let step_x = 1;
        let side_dist_x = (map_x as f32 + 1.0 - player.pos.x / block_size as f32) * delta_dist_x;
        (side_dist_x, step_x)
    };
    
    let (mut side_dist_y, step_y) = if dy < 0.0 {
        let step_y = -1;
        let side_dist_y = (player.pos.y / block_size as f32 - map_y as f32) * delta_dist_y;
        (side_dist_y, step_y)
    } else {
        let step_y = 1;
        let side_dist_y = (map_y as f32 + 1.0 - player.pos.y / block_size as f32) * delta_dist_y;
        (side_dist_y, step_y)
    };
    
    let mut hit = false;
    let mut side = false; // false si es pared NS, true si es pared EW
    let mut wall_type = ' ';
    
    // DDA - Solo salta de cuadrícula en cuadrícula
    while !hit {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = false;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = true;
        }
        
        // Verificar límites y colisión
        if map_y >= 0 && map_y < maze.len() as i32 && 
           map_x >= 0 && map_x < maze[0].len() as i32 {
            wall_type = maze[map_y as usize][map_x as usize];
            if wall_type != ' ' {
                hit = true;
            }
        } else {
            hit = true;
            wall_type = '#'; // Pared por defecto fuera de límites
        }
    }
    
    // Calcular distancia perpendicular
    let perp_wall_dist = if !side {
        (map_x as f32 - player.pos.x / block_size as f32 + (1.0 - step_x as f32) / 2.0) / dx
    } else {
        (map_y as f32 - player.pos.y / block_size as f32 + (1.0 - step_y as f32) / 2.0) / dy
    };
    
    let distance = perp_wall_dist * block_size as f32;
    
    // Calcular coordenada de textura
    let wall_x = if !side {
        player.pos.y + perp_wall_dist * dy
    } else {
        player.pos.x + perp_wall_dist * dx
    };
    
    let wall_x = wall_x - wall_x.floor();
    let tx = (wall_x * 128.0) as usize;
    
    Intersect {
        distance,
        impact: wall_type,
        tx: tx.min(127),
        side
    }
}
