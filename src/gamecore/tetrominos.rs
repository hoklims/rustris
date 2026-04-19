use strum_macros::EnumIter;
use macroquad::color::{ BLUE, GREEN, LIGHTGRAY, MAGENTA, RED, YELLOW, Color };

use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };
use crate::gamecore::masks::{ MASKS_I, MASKS_J, MASKS_L, MASKS_O, MASKS_S, MASKS_T, MASKS_Z } ;

#[derive(EnumIter)]
pub enum TetrominoType { I, O, T, L, J, Z, S }

pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 0.0);

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i8,
    pub y: i8
}

impl std::ops::Add for Coord {

    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{ x: self.x + rhs.x,
               y: self.y + rhs.y }
    }
}

impl std::ops::AddAssign for Coord {

    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;    
    }    
}

impl std::ops::Add for &Coord {

    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{ x: self.x + rhs.x,
               y: self.y + rhs.y }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    blocks_masks: &'static[[Coord; 4]],
    current_mask_idx: usize,
    pub color: Color,
    pub mask: &'static[Coord; 4],
    pub next_mask: &'static[Coord; 4]
}

impl Tetromino {

    pub fn new(t_type: TetrominoType) -> Tetromino {

        match t_type {

            TetrominoType::O => { 
                
                Tetromino { blocks_masks: MASKS_O,
                            current_mask_idx: 0,
                            color: CYAN,
                            mask: &MASKS_O[0],
                            next_mask: &MASKS_O[0]}}

            TetrominoType::I => { 

                Tetromino { blocks_masks: MASKS_I,
                            current_mask_idx: 0,
                            color: RED,
                            mask: &MASKS_I[0],
                            next_mask: &MASKS_I[1] }}

            TetrominoType::J => {                                
                
                Tetromino { blocks_masks: MASKS_J,
                            current_mask_idx: 0,
                            color: MAGENTA,
                            mask: &MASKS_J[0],
                            next_mask: &MASKS_J[1] }}

            TetrominoType::S => { 
                
                Tetromino { blocks_masks: MASKS_S,
                            current_mask_idx: 0,
                            color: BLUE,
                            mask: &MASKS_S[0],
                            next_mask: &MASKS_S[1] }}

            TetrominoType::Z => { 
                
                Tetromino { blocks_masks: MASKS_Z,
                            current_mask_idx: 0,
                            color: GREEN,
                            mask: &MASKS_Z[0],
                            next_mask: &MASKS_Z[1] }}

            TetrominoType::L => { 
                
                Tetromino { blocks_masks: MASKS_L,
                            current_mask_idx: 0,
                            color: YELLOW,
                            mask: &MASKS_L[0],
                            next_mask: &MASKS_L[1] }}

            TetrominoType::T => { 
                
                Tetromino { blocks_masks: MASKS_T,
                            current_mask_idx: 0,
                            color: LIGHTGRAY,
                            mask: &MASKS_T[0],
                            next_mask: &MASKS_T[1] }}
                            
        }
    }

    pub fn update_mask_and_next_one(&mut self) -> () {

        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { self.current_mask_idx  = 0; }
        else 
            { self.current_mask_idx += 1; }

        self.mask = &self.blocks_masks[self.current_mask_idx];

        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { self.next_mask = &self.blocks_masks[0]; }
        else 
            { self.next_mask = &self.blocks_masks[self.current_mask_idx + 1] }

    } 

    pub fn get_width(&self) -> i8 
        { self.mask.iter().map(|x: &Coord| x.x).max().unwrap() }

    pub fn get_height(&self) -> i8 
        { println!("mask {:?}", self.mask); self.mask.iter().map(|x: &Coord| x.y).max().unwrap() }

    pub fn get_init_coord(&self) -> Coord {
        let tet_height: i8 = self.get_height();
        let tet_width: i8 = self.get_width(); 
        println!("tet height {:?}", &tet_height);
        println!("new tet coords, {:?}", Coord { x: (GRID_WIDTH - tet_width - 1) / 2, 
                                                 y:  GRID_HEIGHT - tet_height - 1 });
        Coord { x: (GRID_WIDTH - tet_width - 1) / 2, 
                y:  GRID_HEIGHT - tet_height - 1 }
    }

}

