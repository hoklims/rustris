use macroquad::color::GRAY;
use macroquad::prelude::ImageFormat;
use macroquad::texture::Texture2D;
use macroquad::ui::{ Skin, widgets, Style };
use macroquad::ui::{ root_ui, Ui, hash };
use macroquad::math::{ Vec2, vec2 };
use crate::render::window::Window;

pub enum UIAction {
    Up,
    Right,
    Down,
    Left    
}

pub struct ButtonRenderer {
    button_skin: Skin,
    up_button_texture: Texture2D,
    right_button_texture: Texture2D,
    down_button_texture: Texture2D,
    left_button_texture: Texture2D
}

impl ButtonRenderer {

    pub fn new(arrow_up_bytes: &[u8],
               arrow_right_bytes: &[u8],
               arrow_down_bytes: &[u8],
               arrow_left_bytes: &[u8]) -> ButtonRenderer {

        let button_skin: Skin =  {

            let button_style: Style = root_ui().style_builder().color(GRAY).build();
            let window_style: Style = root_ui().style_builder().color(GRAY).build();

            Skin { button_style,
                   window_style,
                   ..root_ui().default_skin().clone() }
            };

        ButtonRenderer {

            button_skin,

            up_button_texture: Texture2D::from_file_with_format(arrow_up_bytes,
                                                                Some(ImageFormat::Png)), 

            right_button_texture: Texture2D::from_file_with_format(arrow_right_bytes,
                                                                   Some(ImageFormat::Png)),

            down_button_texture: Texture2D::from_file_with_format(arrow_down_bytes, 
                                                                  Some(ImageFormat::Png)),

            left_button_texture: Texture2D::from_file_with_format(arrow_left_bytes, 
                                                                  Some(ImageFormat::Png))
        }
    }

    pub fn render_buttons(&self, window: &Window) -> Option<UIAction> {

        let button_window_position: Vec2 = vec2(window.buttons_area_origin.0,
                                                window.buttons_area_origin.1);
        
        let button_size: Vec2 = vec2(window.block_size * 2.0 , window.block_size * 2.0);

        let window_size: Vec2 = vec2(6.0 * window.block_size, 6.0 * window.block_size);

        let up_button_position: Vec2 = vec2(2.0 * window.block_size, 0.0);
        let right_button_position: Vec2 = vec2(4.0 * window.block_size, 2.0 * window.block_size);
        let down_button_position: Vec2 = vec2(2.0 * window.block_size, 4.0 * window.block_size);
        let left_button_position: Vec2 = vec2(0.0, 2.0 * window.block_size);

        let mut result: Option<UIAction> = None;

        if cfg!(target_arch = "aarch64") {
        
            root_ui().push_skin(&self.button_skin);

            root_ui().window(hash!(), button_window_position, window_size, |ui: &mut Ui | {

                if widgets::Button::new(self.up_button_texture.clone())
                    .position(up_button_position)
                    .size(button_size)
                    .ui(ui)
                        { result = Some(UIAction::Up); }

                if widgets::Button::new(self.right_button_texture.clone())
                    .position(right_button_position)
                    .size(button_size)
                    .ui(ui)
                        { result = Some(UIAction::Right); }


                if widgets::Button::new(self.down_button_texture.clone())
                    .position(down_button_position)
                    .size(button_size)
                    .ui(ui)
                        { result = Some(UIAction::Down); }

                if widgets::Button::new(self.left_button_texture.clone())
                    .position(left_button_position)
                    .size(button_size)
                    .ui(ui)
                        { result = Some(UIAction::Left); }
                        
            });

            root_ui().pop_skin();

        }

        result
    }

}