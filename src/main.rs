use std::time::SystemTime;

use macroquad::{ prelude::BLACK, 
                 text::{ Font, load_ttf_font_from_bytes }, 
                 window::{ clear_background, next_frame },
                 rand::srand };
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::GameGrid, 
             render::{ background_rendering::render_background, 
                       gamegrid_rendering::render_gamegrid, 
                       score_rendering::display_score, 
                       window::Window },
             state::gamegrid_manager::GameGridManager };

const FONT_FILE: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");

#[macroquad::main("Rustris")]
async fn main() {
    
    srand(SystemTime::now().elapsed().unwrap().subsec_nanos() as u64);
    let font: Font = load_ttf_font_from_bytes(FONT_FILE).unwrap();
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
