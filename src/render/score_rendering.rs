use macroquad::{ color::YELLOW, 
                 text::{ Font, TextDimensions, TextParams, draw_text_ex, measure_text }};

use crate::render::window::Window;

pub fn display_score(window :&Window, score: usize, font: &Font) {

    let font_size: u16 = window.block_size as u16;

    let size: TextDimensions = measure_text("SCORE",
                                            Some(font),
                                            font_size,
                                            1.0);
    
    let menu_area_width: f32 = window.score_area_limit.0 - window.score_area_origin.0;

    let score_x_offset: f32 = (menu_area_width - size.width) / 2.0 ;

    let score_title_coord: (f32, f32) = (window.score_area_origin.0 + score_x_offset,
                                         window.score_area_origin.1 + window.block_size * 1.5);

    let text_params: TextParams<'_> = TextParams{font: Some(&font),
                                                 color: YELLOW,
                                                 font_size: font_size,
                                                 font_scale: 1.0,
                                                 font_scale_aspect: 1.0,
                                                 rotation: 0.0};

    draw_text_ex("SCORE",
                 score_title_coord.0, 
                 score_title_coord.1, 
                 text_params);

}