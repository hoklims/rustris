#[allow(dead_code)]
use std::time::SystemTime;

use macroquad::{ prelude::BLACK, rand::srand, 
                 text::{ Font, load_ttf_font_from_bytes }, 
                 window::{ clear_background, next_frame } };
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::GameGrid, 
             render::{ background_rendering::render_background, 
                       gamegrid_rendering::render_gamegrid, 
                       score_rendering::display_score, 
                       window::Window,
                       buttons::ButtonRenderer },
             state::gamegrid_manager::GameGridManager };

const FONT_SCORE: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");

const ARROW_UP_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_up_gray.png");
const ARROW_RIGHT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_right_gray.png");
const ARROW_DOWN_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_down_gray.png");
const ARROW_LEFT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_left_gray.png");

#[macroquad::main("Rustris")]
async fn main() {
    
    srand(SystemTime::now().elapsed().unwrap().subsec_nanos() as u64);
    let font_score: Font = load_ttf_font_from_bytes(FONT_SCORE).unwrap();
    let mut window: Window = Window::new();
    let mut gamegrid: GameGrid = GameGrid::new();
    let mut gamegrid_manager: GameGridManager = GameGridManager::new();
    let button_renderer: ButtonRenderer = ButtonRenderer::new(ARROW_UP_PNG_BYTES,
                                                              ARROW_RIGHT_PNG_BYTES,
                                                              ARROW_DOWN_PNG_BYTES,
                                                              ARROW_LEFT_PNG_BYTES);

    loop {
        
        window.refresh_window_if_needed();
        clear_background(BLACK);
        render_background(&window);
        button_renderer.render_buttons(&window);
        gamegrid_manager.get_and_apply_player_input(&mut gamegrid).unwrap();
        render_gamegrid(&gamegrid, &window);
        display_score(&window, gamegrid_manager.score, gamegrid_manager.level, &font_score);
        next_frame().await

    }
}
