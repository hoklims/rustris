use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };
use macroquad::window::{ screen_width, screen_height };

pub const MENU_HEIGHT: i8 = 4;
pub const MENU_WIDTH: i8 = 5;
const HW_SCREEN_RATIO: f32 = (GRID_HEIGHT + 2) as f32 / 
                             (GRID_WIDTH + MENU_WIDTH + 3) as f32;

pub const HWSCREEN_DIM_BLOCKS: (i8, i8) = (GRID_HEIGHT + 2, GRID_WIDTH + MENU_WIDTH + 3) ;

pub struct Window {
    pub screen_dim: (f32, f32), // order of dim must be [height, width]
    pub display_dim: (f32, f32),
    pub display_origin: (f32, f32), // coordinates are in order (x, y), top left
    pub block_size: f32,
    pub score_area_origin: (f32, f32), // (x, y), top left too
    pub score_area_limit: (f32, f32)
}

impl Window {
    
    pub fn new() -> Window {
        let screen_dim: (f32, f32) = ( screen_height(), screen_width() );
        let display_dim: (f32, f32) = Self::compute_display_dim(&screen_dim);
        let display_origin: (f32, f32) = Self::compute_display_origin(&screen_dim, 
                                                                      &display_dim);
        let block_size: f32 = screen_dim.0 / ( (GRID_HEIGHT + 2) as f32 );
        let score_area_origin = Self::compute_score_area_origin(block_size,
                                                                            &display_origin);
        let score_area_limit: (f32, f32) = Self::compute_score_area_limit(block_size,
                                                                          &score_area_origin);
        Window { screen_dim: screen_dim,
                 display_dim: display_dim,
                 display_origin: display_origin,
                 block_size: block_size,
                 score_area_origin: score_area_origin,
                 score_area_limit: score_area_limit }
    }

    fn has_changed(&self) -> bool {
        self.screen_dim != (screen_height(), screen_width())
    }

    fn compute_display_dim(screen_dim: &(f32, f32)) -> (f32, f32) {
        //compute usable display size while respecting window hw ratio
        let (width, height) = screen_dim.clone();

        if width * HW_SCREEN_RATIO > height { // screen too thin
            (width * HW_SCREEN_RATIO, width)
        }
        else if height / HW_SCREEN_RATIO < width { // screen too wide
            (height / HW_SCREEN_RATIO, width)
        }
        else { (height, width) } // window has right size, can use all area
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

    fn compute_score_area_limit(block_size: f32, score_area_origin: &(f32, f32)) -> (f32, f32) {
        (score_area_origin.0 + (MENU_WIDTH as f32) * block_size,
         score_area_origin.1 + (MENU_HEIGHT as f32) * block_size)
    }

    pub fn refresh_window_if_needed(&mut self) -> () {

        if self.has_changed() {

            self.screen_dim = ( screen_height(), screen_width() );
            self.display_dim = Self::compute_display_dim(&self.screen_dim);
            self.display_origin = Self::compute_display_origin(&self.screen_dim, 
                                                               &self.display_dim);
            self.block_size = self.screen_dim.0 / ( (GRID_HEIGHT + 2) as f32 );
        }
    }

}

