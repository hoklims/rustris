use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };
use macroquad::window::{ screen_width, screen_height };

const HW_SCREEN_RATIO: f32 = 22.0 / 17.0;
const HW_GRID_RATIO: f32 = GRID_HEIGHT as f32 / GRID_WIDTH as f32;
const HW_GRID_WBORDER_RATIO: f32 = (GRID_HEIGHT as f32 + 2.0) / (GRID_WIDTH as f32 + 2.0);

struct Window {
    screen_dim: (f32, f32), // order of dim must be [height, width]
    display_dim: (f32, f32),
    display_origin: (f32, f32) // coordinates are in order (x, y)
}

impl Window {
    
    fn new() -> Window {
        let screen_dim: (f32, f32) = ( screen_height(), screen_width() );
        let display_dim: (f32, f32) = Self::compute_display_dim(&screen_dim);
        let display_origin: (f32, f32) = Self::compute_display_origin(&screen_dim, 
                                                                      &display_dim);
        Window { screen_dim: screen_dim,
                 display_dim: display_dim,
                 display_origin }
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
            (height / HW_GRID_RATIO, width)
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

    fn refresh_window_if_needed(&mut self) -> () {

        if self.has_changed() {

            self.screen_dim = ( screen_height(), screen_width() );
            self.display_dim = Self::compute_display_dim(&self.screen_dim);
            self.display_origin = Self::compute_display_origin(&self.screen_dim, 
                                                               &self.display_dim);
        }
    }
}

