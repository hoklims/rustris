use macroquad::{ color::YELLOW, 
                 text::{ Font, TextDimensions, TextParams, draw_text_ex, measure_text }};

use crate::render::window::Window;

pub fn display_score(window :&Window, score: usize, font: &Font) {

    let title_font_size: u16 = window.block_size as u16;

    let title_size: TextDimensions = measure_text("SCORE",
                                                  Some(font),
                                                  title_font_size,
                                                  1.0);
    
    let menu_area_width: f32 = window.score_area_limit.0 - window.score_area_origin.0;

    let score_title_x_offset: f32 = (menu_area_width - title_size.width) / 2.0 ;
    let score_title_y_offset: f32 = (window.block_size * 2.0 - title_size.height) / 2.0;

    let score_title_coord: (f32, f32) = (window.score_area_origin.0 + score_title_x_offset,
                                         window.score_area_origin.1 + score_title_y_offset);
    

    let title_text_params: TextParams<'_> = TextParams{ font: Some(&font),
                                                        color: YELLOW,
                                                        font_size: title_font_size,
                                                        font_scale: 1.0,
                                                        font_scale_aspect: 1.0,
                                                        rotation: 0.0 };

    draw_text_ex("SCORE",
                 score_title_coord.0, 
                 score_title_coord.1, 
                 title_text_params);

    let score_fmt: String = format!("{:010}", score);
    let score_value_font_size: u16 = title_font_size / 2;

    let value_text_size: TextDimensions = measure_text(&score_fmt,
                                                       Some(font), 
                                                       score_value_font_size, 
                                                       1.0 );

    let value_x_offset: f32 = (menu_area_width - value_text_size.width) / 2.0;
    let value_y_offset: f32 = (1.5 * window.block_size + value_text_size.height / 2.0);


    let score_value_coordinates: (f32, f32) = (window.score_area_origin.0 + value_x_offset,
                                               window.score_area_origin.1 + value_y_offset);
    
    let value_text_params: TextParams<'_> = TextParams { font: Some(&font),
                                                         color: YELLOW,
                                                         font_size: score_value_font_size,
                                                         font_scale: 1.0,
                                                         font_scale_aspect: 1.0,
                                                         rotation: 0.0 };

    draw_text_ex(&score_fmt,
                 score_value_coordinates.0,
                 score_value_coordinates.1,
                 value_text_params);
}