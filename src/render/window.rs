use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };
use macroquad::window::{ screen_width, screen_height };

pub const MENU_HEIGHT: i8 = 3;
pub const MENU_WIDTH: i8 = 5;

pub const HWSCREEN_DIM_BLOCKS: (i8, i8) = (GRID_HEIGHT + 2, GRID_WIDTH + MENU_WIDTH + 3) ;

pub struct Window {
    pub screen_dim: (f32, f32), // order of dim must be [height, width]
    pub display_dim: (f32, f32),
    pub display_origin: (f32, f32), // coordinates are in order (x, y), top left
    pub buttons_area_origin: (f32, f32), 
    pub block_size: f32,
    pub score_area_origin: (f32, f32), // (x, y), top left too
}

impl Window {
    
    pub fn new() -> Window {
        let screen_dim: (f32, f32) = ( screen_height(), screen_width() );
        let display_dim: (f32, f32) = Self::compute_display_dim(&screen_dim);
        let display_origin: (f32, f32) = Self::compute_display_origin(&screen_dim, 
                                                                      &display_dim);

        let block_size: f32 = Self::compute_block_size(&screen_dim);

        let score_area_origin: (f32, f32) = Self::compute_score_area_origin(block_size,
                                                                            &display_origin);

        let button_area_origin: (f32, f32) = Self::compute_buttons_area_origin(block_size, &display_origin);

        Window { screen_dim: screen_dim,
                 display_dim: display_dim,
                 display_origin: display_origin,
                 block_size: block_size,
                 score_area_origin: score_area_origin,
                 buttons_area_origin: button_area_origin }
    }

    fn has_changed(&self) -> bool {
        self.screen_dim != (screen_height(), screen_width())
    }

    fn compute_display_dim(screen_dim: &(f32, f32)) -> (f32, f32) {
        let block_size: f32 = Self::compute_block_size(screen_dim);
        (block_size * HWSCREEN_DIM_BLOCKS.0 as f32,
         block_size * HWSCREEN_DIM_BLOCKS.1 as f32)
    }

    fn compute_block_size(screen_dim: &(f32, f32)) -> f32 {
        let height_limited: f32 = screen_dim.0 / HWSCREEN_DIM_BLOCKS.0 as f32;
        let width_limited: f32 = screen_dim.1 / HWSCREEN_DIM_BLOCKS.1 as f32;
        height_limited.min(width_limited)
    }

    fn compute_display_origin(window_size: &(f32, f32), display_dim: &(f32, f32)) -> (f32, f32) {
        let (win_height, win_width) = window_size;
        let (disp_height, disp_width) = display_dim;

        if win_width > disp_width // screen is too wide
            { ( ((win_width - disp_width) / 2.0), 0.0 ) }

        else if win_height > disp_height // screen is too thin
            { ( 0.0, ((win_height - disp_height) / 2.0) ) }

        else { (0.0, 0.0) }
    }

    fn compute_score_area_origin(block_size: f32, display_origin: &(f32, f32)) -> (f32, f32) {
        ((GRID_WIDTH + 2) as f32 * block_size + display_origin.0, 
         block_size + display_origin.1)
    }

    fn compute_buttons_area_origin(block_size: f32, display_origin: &(f32, f32)) -> (f32, f32) {
        (display_origin.0 + block_size * (GRID_WIDTH + 2) as f32,
         display_origin.1 + block_size * (MENU_HEIGHT + 7) as f32)
    }

    pub fn refresh_window_if_needed(&mut self) -> () {

        if self.has_changed() {

            self.screen_dim = ( screen_height(), screen_width() );
            self.display_dim = Self::compute_display_dim(&self.screen_dim);
            self.display_origin = Self::compute_display_origin(&self.screen_dim, 
                                                               &self.display_dim);

            self.block_size = Self::compute_block_size(&self.screen_dim);

            self.score_area_origin = Self::compute_score_area_origin(self.block_size,
                                                                     &self.display_origin);

            self.buttons_area_origin = Self::compute_buttons_area_origin(self.block_size,
                                                                         &self.display_origin);

        }
    }
}

#[cfg(test)]
mod tests {
    use super::Window;

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
    }

    #[test]
    fn portrait_layout_fits_the_full_game_width() {
        let screen: (f32, f32) = (2340.0, 1080.0);
        let block_size: f32 = Window::compute_block_size(&screen);
        let display: (f32, f32) = Window::compute_display_dim(&screen);
        let origin: (f32, f32) = Window::compute_display_origin(&screen, &display);

        assert_close(block_size, 60.0);
        assert_eq!(display, (1320.0, 1080.0));
        assert_eq!(origin, (0.0, 510.0));
    }

    #[test]
    fn landscape_layout_fits_the_full_game_height() {
        let screen: (f32, f32) = (1080.0, 2340.0);
        let block_size: f32 = Window::compute_block_size(&screen);
        let display: (f32, f32) = Window::compute_display_dim(&screen);
        let origin: (f32, f32) = Window::compute_display_origin(&screen, &display);

        assert_close(block_size, 1080.0 / 22.0);
        assert_close(display.0, 1080.0);
        assert_close(display.1, 1080.0 / 22.0 * 18.0);
        assert_close(origin.0, (2340.0 - display.1) / 2.0);
        assert_close(origin.1, 0.0);
    }
}

