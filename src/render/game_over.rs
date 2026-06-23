use macroquad::{ color::YELLOW, 
                 text::{ Font, TextDimensions, TextParams, draw_text_ex, measure_text }};

use crate::render::window::Window;

pub fn display_game_over(window: &Window, font: &Font) -> () {
    
    let game_over_font_size: u16 = window.block_size as u16 * 2 ;
    let game_over_text_size: TextDimensions = measure_text(
        "GAME OVER", 
        Some(font), 
        game_over_font_size, 
        1.0
    );

    let game_over_text_position: (f32, f32) = {
        let height: f32 = window.screen_dim.0 / 3.0; 
        let width: f32 = (window.screen_dim.1 - game_over_text_size.width) / 2.0;
        (height, width)
    };

    let game_over_text_params: TextParams<'_> = TextParams { 
        font: Some(&font),
        color: YELLOW,
        font_size: game_over_font_size,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        rotation: 0.0 
    };

    draw_text_ex(
        "GAME OVER",
        game_over_text_position.1,
        game_over_text_position.0,
        game_over_text_params
    );

    let additional_text: &str = {
        if cfg!(target_arch = "arm") 
            { "TOUCH ANY BUTTON TO CONTINUE" }
        else 
            { "PRESS ANY ARROW KEY TO CONTINUE" }
    };

    let additional_text_font_size: u16 = window.block_size as u16 / 3 * 2;

    let additional_text_size: TextDimensions = measure_text(
        additional_text, 
        Some(font), 
        additional_text_font_size, 
        1.0);

    let additional_text_position: (f32, f32) = {
        let height: f32 = game_over_text_position.0 + game_over_text_size.height;
        let width: f32 = (window.screen_dim.1 - additional_text_size.width) /2.0;
        (height, width)
    };

    let additional_text_params: TextParams<'_> = TextParams { 
        font: Some(&font),
        color: YELLOW,
        font_size: additional_text_font_size,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        rotation: 0.0 
    };

    draw_text_ex(
        additional_text, 
        additional_text_position.1, 
        additional_text_position.0, 
        additional_text_params
    );

}