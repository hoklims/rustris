use crate::{gamecore::game_grid::GRID_WIDTH, render::{ block_rendering::render_block, window::{ HWSCREEN_DIM_BLOCKS, MENU_HEIGHT, Window }}} ;
use macroquad::{ prelude::GRAY };



pub fn render_background(window: &Window) -> () {
    /* background is one line of grey blocks as border and six columns
       at right for menu, with a window of five blocks over 5 lines for score */
    // upper border
    for col in  0..HWSCREEN_DIM_BLOCKS.1 {
        render_block(&((col as f32 * window.block_size) , 0.0), 
                     GRAY, 
                     window,
                     false);
    }

    // menu lines
    for row in 1..(MENU_HEIGHT + 1) {
        for col in 0..HWSCREEN_DIM_BLOCKS.1 {
            if !(1..(GRID_WIDTH + 1)).contains(&col) &&
               !((GRID_WIDTH + 3)..(HWSCREEN_DIM_BLOCKS.1 - 1)).contains(&col)
            { render_block(&( col as f32 * window.block_size, row as f32 * window.block_size), 
                           GRAY, 
                           window, 
                           false); }
        }
    }

    // remaining lines until bottom border
    for row in (MENU_HEIGHT + 1)..(HWSCREEN_DIM_BLOCKS.0 - 1) {
        for col in 0..HWSCREEN_DIM_BLOCKS.1 {
            if !(1..(GRID_WIDTH + 1)).contains(&col)
            { render_block(&( col as f32 * window.block_size, row as f32 * window.block_size), 
                           GRAY, 
                           window, 
                           false); }
        }
    }

    // bottom border
    for col in  0..HWSCREEN_DIM_BLOCKS.1 {
        render_block(&((col as f32 * window.block_size) , 
                       (HWSCREEN_DIM_BLOCKS.0 - 1) as f32 * window.block_size),
                     GRAY, 
                     window,
                     false);
    }

    
}