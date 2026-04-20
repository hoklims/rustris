use macroquad::text::{ Font, measure_text };

use crate::render::window::Window;

pub fn display_score(window :&Window, score: usize, font: Font) {
    let size = measure_text("SCORE",
                                            Some(&font),
                                            window.block_size as u16,
                                            1.0);
}