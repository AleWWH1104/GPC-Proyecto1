use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: usize) {
    let mut d = 0.0;

    framebuffer.set_current_color(Color::WHITESMOKE);

    loop {
        let cos = d * player.a.cos();
        let sin = d * player.a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // convert pixels to a position in the maze
        let i = x  / block_size;
        let j = y / block_size;

        // if the current item is not a space,
        // we have hit a wall and we stop
        if maze[j][i] != ' ' {
            break;
        }

        framebuffer.set_pixel(x as u32 , y as u32);

        d += 10.0;
    }
}
