use std::ops::Not;

use macroquad::color::BLACK;

use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH, GameGrid };
use crate::gamecore::tetrominos::Coord;
use crate::render::block_rendering::{ render_block, render_square };
use crate::render::window::Window;

pub fn render_gamegrid(gamegrid: &GameGrid, window: &Window) -> () {

    let gamegrid_display_origin: (f32, f32) = ( window.block_size,
                                                window.block_size );

    // rendering fixed blocks
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            // gamegrid has bottom left index, macroquad needs top left
            let mq_coord: (f32, f32) = 
                ( col as f32 * window.block_size + gamegrid_display_origin.0,
                  (GRID_HEIGHT - row - 1) as f32 * window.block_size + gamegrid_display_origin.1 );

            match gamegrid.grid[row as usize][col as usize] {
                Some(color) => { render_block(&mq_coord, color, window, false); }
                None => { render_square(&mq_coord, BLACK, window); }
            }
        }
    }

    // rendering tetromino
    if gamegrid.current_tetromino.is_none().not() {
        for coord in gamegrid.current_tetromino.unwrap().mask
                                .map(|x: Coord| {x + gamegrid.tet_coord}) {
            
            let mq_coord: (f32, f32) = 
                ( coord.x as f32 * window.block_size + gamegrid_display_origin.0,
                (GRID_HEIGHT - coord.y - 1) as f32 * window.block_size + gamegrid_display_origin.1 );
            
            render_block(
                &mq_coord, 
                gamegrid.current_tetromino.unwrap().color, 
                window, 
                false)    
        }
    }

}