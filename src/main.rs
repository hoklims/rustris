use std::time::{Duration, Instant, SystemTime };

use macroquad::{ input::{ is_key_pressed, KeyCode }, 
                 prelude::BLACK, 
                 rand::srand, 
                 text::{ Font, load_ttf_font_from_bytes }, 
                 window::{ clear_background, next_frame } };
mod gamecore;
mod state;
mod render;
use crate::{ gamecore::game_grid::{ GameGrid, GridError }, 
             render::{ background_rendering::render_background, 
             buttons::{ ButtonRenderer, UIAction }, 
             game_over::display_game_over, 
             gamegrid_rendering::render_gamegrid, 
             score_rendering::display_score, window::Window }, 
             state::gamegrid_manager::GameGridManager };

const FONT_SCORE: &[u8] = include_bytes!("../assets/square_sans_serif_7.ttf");

const ARROW_UP_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_up_gray.png");
const ARROW_RIGHT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_right_gray.png");
const ARROW_DOWN_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_down_gray.png");
const ARROW_LEFT_PNG_BYTES: &[u8] = include_bytes!("../assets/arrow_left_gray.png");


#[macroquad::main("Rustris")]
async fn main() {
    
    srand(SystemTime::now().elapsed().unwrap().subsec_nanos() as u64);
    let font: Font = load_ttf_font_from_bytes(FONT_SCORE).unwrap();
    let mut window: Window = Window::new();

    let button_renderer: ButtonRenderer = ButtonRenderer::new(
        ARROW_UP_PNG_BYTES,
        ARROW_RIGHT_PNG_BYTES,
        ARROW_DOWN_PNG_BYTES,
        ARROW_LEFT_PNG_BYTES
        );

    let mut gamegrid: GameGrid = GameGrid::new();
    let mut gamegrid_manager: GameGridManager = GameGridManager::new();
    let mut game_over_time: Option<Instant> = None;

    loop {
        
        window.refresh_window_if_needed();
        clear_background(BLACK);
        render_background(&window);
        let ui_action: Option<UIAction> = button_renderer.render_buttons(&window);
        let ui_action_result: Result<(), GridError> = gamegrid_manager.apply_ui_input(&mut gamegrid, 
                                                                                      ui_action);
        let key_action_result: Result<(), GridError> = gamegrid_manager.get_and_apply_player_input(&mut gamegrid);
        render_gamegrid(&gamegrid, &window);
        display_score(&window, gamegrid_manager.score, gamegrid_manager.level, &font);

        
        match (ui_action_result, key_action_result, &gamegrid.current_tetromino) {

            (.., None) |
            (Err(GridError::CannotAllocateNewTet), ..) |
            (_, Err(GridError::CannotAllocateNewTet), _) => { 
                
                display_game_over(&window, &font);

                if game_over_time.is_none() {
                    game_over_time = Some(Instant::now());
                }

                let any_key_pressed: bool = [is_key_pressed(KeyCode::Up),
                                             is_key_pressed(KeyCode::Right),
                                             is_key_pressed(KeyCode::Down),
                                             is_key_pressed(KeyCode::Left)].iter().any(|x| *x);

                if any_key_pressed && game_over_time.unwrap().elapsed() > Duration::from_secs(3) {

                        gamegrid = GameGrid::new();
                        gamegrid_manager = GameGridManager::new();
                        game_over_time = None;

                    }

                next_frame().await; 
            },

            (Err(error), ..) => panic!("ui returned {error:?}"),
            (_, Err(error), _) => panic!("key returned {error:?}"),
             _ => next_frame().await 

        }   

    }

}
