use std::vec::Vec;
use strum_macros::EnumIter;
use macroquad::color::{ RED, BLUE, MAGENTA, LIGHTGRAY, PINK, GREEN, Color };

use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };

#[derive(EnumIter)]
pub enum TetronimoType { I, O, T, L, J, Z, S }

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i8,
    pub y: i8
}

impl std::ops::Add for Coord {

    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{
                x: self.x + rhs.x,
                y: self.y + rhs.y
        }
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y    
    }    
}

#[derive(Debug)]
pub struct Tetronimo {
    blocks_masks: Vec<[Coord; 4]>,
    current_mask_idx: usize,
    pub color: Color,
}

impl Clone for Tetronimo {

    fn clone(&self) -> Self {
        
        Tetronimo { blocks_masks: self.blocks_masks.to_vec(),
                    current_mask_idx: self.current_mask_idx,
                    color: self.color }

    }

}

impl Tetronimo {

    pub fn new(t_type: TetronimoType) -> Tetronimo {

        match t_type {

            TetronimoType::O => { Tetronimo { blocks_masks: vec![[Coord{ x:  0, y: 0 }, 
                                                                  Coord{ x:  1, y: 0 }, 
                                                                  Coord{ x:  1, y: 1 }, 
                                                                  Coord{ x:  0, y: 1 }]],
                                              current_mask_idx: 0,
                                              color: BLUE }}

            TetronimoType::I => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 0 }, 
                                                                  Coord{ x: 0, y: 1 }, 
                                                                  Coord{ x: 0, y: 2 }, 
                                                                  Coord{ x: 0, y: 3 }],
                                                                 [Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 2, y: 0 },
                                                                  Coord{ x: 3, y: 0 }]],
                                              current_mask_idx: 0,
                                              color: RED }}

            TetronimoType::J => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 2 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 }],
                                                                 [Coord{ x: 0, y: 2 },
                                                                  Coord{ x: 1, y: 2 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 }]],
                                              current_mask_idx: 0,
                                              color: LIGHTGRAY }}

            TetronimoType::S => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 2, y: 1 }],
                                                                 [Coord{ x: 0, y: 2 },
                                                                  Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 }]],
                                              current_mask_idx: 0,
                                              color: GREEN }}

            TetronimoType::Z => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 2, y: 0 }],
                                                                 [Coord{ x: 1, y: 2 },
                                                                  Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 0, y: 0 }]],
                                              current_mask_idx: 0,
                                              color: PINK }}

            TetronimoType::L => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 2 },
                                                                  Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 0 }],
                                                                 [Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 2, y: 0 },
                                                                  Coord{ x: 2, y: 1 }]],
                                              current_mask_idx: 0,
                                              color: MAGENTA }}

            TetronimoType::T => { Tetronimo { blocks_masks: vec![[Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 2, y: 0 },
                                                                  Coord{ x: 1, y: 1 }],
                                                                 [Coord{ x: 1, y: 2 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 1, y: 0 },
                                                                  Coord{ x: 0, y: 1 }],
                                                                 [Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 1, y: 1 },
                                                                  Coord{ x: 2, y: 1 },
                                                                  Coord{ x: 1, y: 0 }],
                                                                 [Coord{ x: 0, y: 0 },
                                                                  Coord{ x: 0, y: 1 },
                                                                  Coord{ x: 0, y: 2 },
                                                                  Coord{ x: 1, y: 1 }]],
                                              current_mask_idx: 0,
                                              color: PINK }}

        }
    }

    pub fn update_mask(&mut self) -> () {
        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { self.current_mask_idx  = 0 }
        else 
            { self.current_mask_idx += 1 }
    } 

    pub fn get_mask<'a>(&'a self) -> &'a [Coord; 4] 
        { &self.blocks_masks[self.current_mask_idx] }

    pub fn get_next_mask<'a>(&'a self) -> &'a [Coord; 4] {
        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { &self.blocks_masks[0] }
        else 
            { &self.blocks_masks[self.current_mask_idx + 1] }

    }

    pub fn get_width(&self) -> i8 
        { self.get_mask().iter().map(|x| x.x).max().unwrap() }

    pub fn get_height(&self) -> i8 
        { self.get_mask().iter().map(|x| x.y).max().unwrap() }

    pub fn get_init_coord(&self) -> Coord {
        let tet_height: i8 = self.get_height();
        let tet_width: i8 = self.get_width(); 
        Coord { x: (GRID_WIDTH - tet_width) / 2, 
                y:  GRID_HEIGHT - tet_height }
    }

}

