#![allow(dead_code)] // REMOVE THIS BEFORE SHIPPING
use macroquad::{ prelude::BLACK,
                 window::{ clear_background, next_frame }};
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::GameGrid, 
             render::{ background_rendering::render_background, 
                       gamegrid_rendering::render_gamegrid, 
                       window::Window },
             state::gamegrid_manager::GameGridManager };
#[macroquad::main("Rustris")]
async fn main() {
    let mut window: Window = Window::new();
    let mut gamegrid: GameGrid = GameGrid::new();
    let mut gamegrid_manager: GameGridManager = GameGridManager::new();
    loop {
        window.refresh_window_if_needed();
        clear_background(BLACK);
        render_background(&window);
        gamegrid_manager.get_and_apply_player_input(&mut gamegrid).unwrap();
        render_gamegrid(&gamegrid, &window);
        next_frame().await
    }
}
