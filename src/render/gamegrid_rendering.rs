use macroquad::color::BLACK;
use macroquad::shapes::draw_rectangle;

use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH, GameGrid };
use crate::render::block_rendering::render_block;
use crate::render::window::Window;

pub fn render_gamegrid(gamegrid: &GameGrid, window: &Window) -> () {

    let gamegrid_display_origin: (f32, f32) = ( window.block_size,
                                                window.block_size );
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            // gamegrid has bottom left index, macroquad needs top left
            let mq_coord: (f32, f32) = 
                ( col as f32 * window.block_size + gamegrid_display_origin.0,
                  (GRID_HEIGHT - row) as f32 * window.block_size + gamegrid_display_origin.1 );

            match gamegrid.grid[row as usize][col as usize] {
                Some(color) => { render_block(&mq_coord, color, window, false); }
                None => { draw_rectangle(mq_coord.0,
                                         mq_coord.1, 
                                         window.block_size, 
                                         window.block_size, 
                                         BLACK) }
            }
        }
    }
}