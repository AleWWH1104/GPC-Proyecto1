use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::player::{self, Player};
use crate::maze::Maze;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer, 
    maze: &Maze, 
    player: &Player, 
    block_size: usize, 
    a: f32, 
    draw: bool
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::BLUEVIOLET);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // convert pixels to a position in the maze
        let i = x  / block_size;
        let j = y / block_size;

        // if the current item is not a space,
        // we have hit a wall and we stop
        if maze[j][i] != ' ' {
            let hitx = x - i * block_size;
            let hity  = y - j * block_size;
            let mut maxhit = hity;

            if 1< hitx && hitx < block_size - 1 {
                maxhit= hitx;
            }
            let x = ((maxhit as f32 * 128.0) / block_size as f32) as usize;
            return Intersect {
                distance: d,
                impact: maze[j][i],
                tx: x,
            }; 
        }

        if draw {
            framebuffer.set_pixel(x as u32, y as u32);
        }

        d += 1.0;
    }
}
