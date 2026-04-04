use std::vec::Vec;
use strum_macros::EnumIter;
use macroquad::color::{ RED, BLUE, MAGENTA, LIGHTGRAY, PINK, GREEN, Color };

#[derive(EnumIter)]
pub enum TetronimoType { I, O, T, L, J, Z, S }

#[derive(Debug, Clone)]
pub struct Coord {
    x: i8,
    y: i8
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

#[derive(Debug)]
pub struct Tetronimo {
    blocks_masks: Vec<[Coord; 4]>,
    current_mask_idx: usize,
    color: Color,
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
}

