use std::time::SystemTime;

use macroquad::{ prelude::BLACK, rand::srand, 
                 text::{ Font, load_ttf_font_from_bytes }, 
                 window::{ clear_background, next_frame } };
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::{ GameGrid, GridError }, 
             render::{ background_rendering::render_background, 
                       buttons::{ ButtonRenderer, UIAction }, 
                       gamegrid_rendering::render_gamegrid, 
                       score_rendering::display_score, window::Window },
                       state::gamegrid_manager::GameGridManager };

const FONT_SCORE: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");

const ARROW_UP_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_up_gray.png");
const ARROW_RIGHT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_right_gray.png");
const ARROW_DOWN_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_down_gray.png");
const ARROW_LEFT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_left_gray.png");

async fn run_game_loop(window: &mut Window, button_renderer: &ButtonRenderer, 
                       gamegrid_manager: &mut GameGridManager, 
                       gamegrid: &mut GameGrid, font_score: &Font) -> Result<(), GridError> {

    loop {
        
        window.refresh_window_if_needed();
        clear_background(BLACK);
        render_background(&window);
        let ui_action: Option<UIAction> = button_renderer.render_buttons(&window);
        gamegrid_manager.apply_ui_input(gamegrid, ui_action)?;
        gamegrid_manager.get_and_apply_player_input(gamegrid)?;
        render_gamegrid(&gamegrid, &window);
        display_score(&window, gamegrid_manager.score, gamegrid_manager.level, &font_score);
        next_frame().await

    }
}

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

        let game_loop_result: Result<(), GridError> = run_game_loop(
            &mut window, 
            &button_renderer, 
            &mut gamegrid_manager, 
            &mut gamegrid, 
            &font_score)
            .await;
        
        match game_loop_result {
            Err(GridError::TetOutsideGrid) | Ok(()) => { continue; },
            _ => { game_loop_result.unwrap() }
        }

    }

}
