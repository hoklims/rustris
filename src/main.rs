#![allow(dead_code)] // REMOVE THIS BEFORE SHIPPING
use macroquad::{ prelude::BLACK, text::{Font, load_ttf_font}, window::{ clear_background, next_frame }};
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::GameGrid, 
             render::{ background_rendering::render_background, gamegrid_rendering::render_gamegrid, score_rendering::display_score, window::Window },
             state::gamegrid_manager::GameGridManager };
#[macroquad::main("Rustris")]
async fn main() {
    let font: Font = load_ttf_font("assets/square_sans_serif_7.ttf").await.unwrap();
    let mut window: Window = Window::new();
    let mut gamegrid: GameGrid = GameGrid::new();
    let mut gamegrid_manager: GameGridManager = GameGridManager::new();
    loop {
        window.refresh_window_if_needed();
        clear_background(BLACK);
        render_background(&window);
        gamegrid_manager.get_and_apply_player_input(&mut gamegrid).unwrap();
        render_gamegrid(&gamegrid, &window);
        display_score(&window, gamegrid_manager.score, gamegrid_manager.level, &font);
        next_frame().await
    }
}
