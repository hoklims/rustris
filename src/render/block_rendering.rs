use crate::render::window::Window;
use macroquad::{ color::Color, prelude::{ draw_rectangle, draw_triangle, Vec2 }};

const EDGE_SIZE_RATIO: f32 = 0.1; // bevel size don't go above 0.5

pub enum RenderingError {
    CoordOutsideWindow
}


pub fn render_block(raw_coord: &(f32, f32),
                    color: Color,
                    window: &Window,
                    ghost:bool) -> () {

    let top_left_coord: (f32, f32) = (window.display_origin.0 + raw_coord.0, 
                                      window.display_origin.1 + raw_coord.1);
    render_edgy_block(&top_left_coord, window, color, ghost);
    
}

pub fn render_edgy_block(top_left_coord: &(f32, f32), window: &Window, color: Color, ghost: bool) -> () {
    /* This function draws an edgy block with triangles for border and square
       for inner color. Coordinates array are clock wise and start at top left */
        let outer_edges_coords: [Vec2; 4] = 
            [ Vec2::new(top_left_coord.0, top_left_coord.1),
              Vec2::new(top_left_coord.0 + window.block_size, top_left_coord.1),
              Vec2::new(top_left_coord.0 + window.block_size, top_left_coord.1 + window.block_size),
              Vec2::new(top_left_coord.0, top_left_coord.1 + window.block_size) ];

        let inner_square_size: f32 = window.block_size * (1.0 - 2.0 * EDGE_SIZE_RATIO);
        let edge_size: f32 = EDGE_SIZE_RATIO * window.block_size;

        let inner_top_left_coord: (f32, f32) = ( top_left_coord.0 + edge_size,
                                                 top_left_coord.1 + edge_size );
        let inner_edges_coords: [Vec2; 4] = 
            [ Vec2::new(inner_top_left_coord.0, inner_top_left_coord.1),
              Vec2::new(inner_top_left_coord.0 + inner_square_size, inner_top_left_coord.1),
              Vec2::new(inner_top_left_coord.0 + inner_square_size, inner_top_left_coord.1 + inner_square_size),
              Vec2::new(inner_top_left_coord.0, inner_top_left_coord.1 + inner_square_size) ];

        draw_edges(&outer_edges_coords, &inner_edges_coords, color);
        if !ghost {
            draw_rectangle(inner_edges_coords[0].x,
                           inner_edges_coords[0].y, 
                           inner_square_size, 
                           inner_square_size, color);
        }
        
    }

fn draw_edges(outer_coords: &[Vec2; 4], inner_coords: &[Vec2; 4], color: Color) -> () {
    /* Witch clockwise inner and outer coords, we will draw two triangles for each edge.
       Edges are done in this order: left, top, right, down */

    let lighter: Color = Color{ r: (color.r * 1.3).min(1.0),
                                g: (color.g * 1.3).min(1.0), 
                                b: (color.b * 1.3).min(1.0), 
                                a: 1.0 };

    let darker: Color = Color { r: color.r * 0.7,
                                g: color.g * 0.7, 
                                b: color.b * 0.7, 
                                a: 1.0 };

    draw_triangle(outer_coords[0], inner_coords[0], outer_coords[3], lighter);
    draw_triangle(outer_coords[3], inner_coords[3], inner_coords[0], lighter);

    draw_triangle(outer_coords[0], inner_coords[0], outer_coords[1], lighter);
    draw_triangle(outer_coords[1], inner_coords[1], inner_coords[0], lighter);

    draw_triangle(outer_coords[1], inner_coords[1], outer_coords[2], darker);
    draw_triangle(outer_coords[2], inner_coords[2], inner_coords[1], darker);

    draw_triangle(outer_coords[2], inner_coords[2], outer_coords[3], darker);
    draw_triangle(outer_coords[3], inner_coords[3], inner_coords[2], darker);
}
