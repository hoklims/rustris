use macroquad::color::{ BLACK, GRAY, WHITE };
use macroquad::text::Font;
use macroquad::ui::Skin;
use macroquad::ui::{ root_ui, hash, widgets::Button, Ui, Style };
use macroquad::math::{ Vec2, vec2 };
use crate::render::window::Window;

pub struct ButtonRenderer {
    skin: Skin
}

impl ButtonRenderer {

    pub fn new(font: &Font) -> ButtonRenderer {

        let window_style: Style = root_ui().style_builder()
                                           .color(GRAY)
                                           .build();
        let button_style: Style = root_ui().style_builder()
                                           .color(WHITE)
                                           .with_font(&font).unwrap()
                                           .text_color(BLACK)
                                           .build();
        let skin = Skin {
            window_style,
            button_style,
            ..root_ui().default_skin()
        };

        ButtonRenderer{ skin }
    }

    pub fn render_buttons(&self, window: &Window) -> () {

        let button_window_position: Vec2 = vec2(window.buttons_area_origin.0,
                                                window.buttons_area_origin.1);
        
        let button_window_size: Vec2  = vec2(window.block_size * 6.0,
                                             window.block_size * 6.0);

        let up_button_position: Vec2 = vec2(2.0 * window.block_size, 
                                            0.0);

        root_ui().push_skin(&self.skin);
        
        root_ui().window(hash!(), button_window_position, button_window_size, |ui: &mut Ui| {
            Button::new("^")
                    .position(up_button_position)
                    .size(vec2(window.block_size * 2.0 , window.block_size*2.0))
                    .ui(ui);
        });
    }
}