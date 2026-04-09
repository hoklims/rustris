use std::vec::Vec;
use strum_macros::EnumIter;
use macroquad::color::{ RED, BLUE, MAGENTA, LIGHTGRAY, GREEN, Color };

use crate::gamecore::game_grid::{ GRID_HEIGHT, GRID_WIDTH };

#[derive(EnumIter)]
pub enum TetronimoType { I, O, T, L, J, Z, S }

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

#[derive(Debug)]
pub struct Tetronimo {
    blocks_masks: Vec<[Coord; 4]>,
    current_mask_idx: usize,
    pub color: Color,
    pub mask: [Coord; 4],
    pub next_mask: [Coord; 4]
}

impl Clone for Tetronimo {

    fn clone(&self) -> Self {
        
        Tetronimo { blocks_masks: self.blocks_masks.clone(),
                    current_mask_idx: self.current_mask_idx,
                    color: self.color,
                    mask: self.mask.clone(),
                    next_mask: self.next_mask.clone() }

    }

}

impl Tetronimo {

    pub fn new(t_type: TetronimoType) -> Tetronimo {

        match t_type {

            TetronimoType::O => { 

                let blocks_mask: Vec<[Coord; 4]> = vec![[Coord{ x:  0, y: 0 }, 
                                                         Coord{ x:  1, y: 0 }, 
                                                         Coord{ x:  1, y: 1 }, 
                                                         Coord{ x:  0, y: 1 }]];
                let (mask, next_mask) = (blocks_mask[0], blocks_mask[0]);
                
                Tetronimo { blocks_masks: blocks_mask,
                            current_mask_idx: 0,
                            color: BLUE,
                            mask: mask,
                            next_mask: next_mask}}

            TetronimoType::I => { 
                
                let blocks_masks: Vec<[Coord; 4]> = vec![[Coord{ x: 0, y: 0 }, 
                                                         Coord{ x: 0, y: 1 }, 
                                                         Coord{ x: 0, y: 2 }, 
                                                         Coord{ x: 0, y: 3 }],
                                                        [Coord{ x: 0, y: 0 },
                                                         Coord{ x: 1, y: 0 },
                                                         Coord{ x: 2, y: 0 },
                                                         Coord{ x: 3, y: 0 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]);

                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: RED,
                            mask: mask,
                            next_mask: next_mask }}

            TetronimoType::J => { 
                
                let blocks_masks: Vec<[Coord; 4]> =  vec![[Coord{ x: 0, y: 0 },
                                                           Coord{ x: 1, y: 2 },
                                                           Coord{ x: 1, y: 1 },
                                                           Coord{ x: 1, y: 0 }],
                                                          [Coord{ x: 0, y: 2 },
                                                           Coord{ x: 1, y: 2 },
                                                           Coord{ x: 1, y: 1 },
                                                           Coord{ x: 1, y: 0 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]);                                           
                
                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: LIGHTGRAY,
                            mask: mask,
                            next_mask: next_mask }}

            TetronimoType::S => { 
                
                let blocks_masks: Vec<[Coord; 4]> = vec![[Coord{ x: 0, y: 0 },
                                                          Coord{ x: 1, y: 1 },
                                                          Coord{ x: 1, y: 0 },
                                                          Coord{ x: 2, y: 1 }],
                                                         [Coord{ x: 0, y: 2 },
                                                          Coord{ x: 0, y: 1 },
                                                          Coord{ x: 1, y: 1 },
                                                          Coord{ x: 1, y: 0 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]); 
                
                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: GREEN,
                            mask: mask,
                            next_mask: next_mask }}

            TetronimoType::Z => { 
                
                let blocks_masks: Vec<[Coord; 4]> = vec![[Coord{ x: 0, y: 1 },
                                                          Coord{ x: 1, y: 1 },
                                                          Coord{ x: 1, y: 0 },
                                                          Coord{ x: 2, y: 0 }],
                                                         [Coord{ x: 1, y: 2 },
                                                          Coord{ x: 0, y: 1 },
                                                          Coord{ x: 1, y: 1 },
                                                          Coord{ x: 0, y: 0 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]); 
                
                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: CYAN,
                            mask: mask,
                            next_mask: next_mask }}

            TetronimoType::L => { 
                
                let blocks_masks: Vec<[Coord; 4]> = vec![[Coord{ x: 0, y: 2 },
                                                          Coord{ x: 0, y: 1 },
                                                          Coord{ x: 0, y: 0 },
                                                          Coord{ x: 1, y: 0 }],
                                                         [Coord{ x: 0, y: 0 },
                                                          Coord{ x: 1, y: 0 },
                                                          Coord{ x: 2, y: 0 },
                                                          Coord{ x: 2, y: 1 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]);
                
                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: MAGENTA,
                            mask: mask,
                            next_mask: next_mask }}

            TetronimoType::T => { 
                
                let blocks_masks: Vec<[Coord; 4]> = vec![[Coord{ x: 0, y: 0 },
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
                                                          Coord{ x: 1, y: 1 }]];

                let (mask, next_mask) = (blocks_masks[0], blocks_masks[1]);
                
                Tetronimo { blocks_masks: blocks_masks,
                            current_mask_idx: 0,
                            color: PINK,
                            mask: mask,
                            next_mask: next_mask }}
        }
    }

    pub fn update_mask_and_next_one(&mut self) -> () {

        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { self.current_mask_idx  = 0; }
        else 
            { self.current_mask_idx += 1; }

        self.mask = self.blocks_masks[self.current_mask_idx];

        if (self.current_mask_idx + 1) == self.blocks_masks.len() 
            { self.next_mask = self.blocks_masks[0]; }
        else 
            { self.next_mask = self.blocks_masks[self.current_mask_idx + 1] }

    } 

    pub fn get_width(&self) -> i8 
        { self.mask.iter().map(|x: &Coord| x.x).max().unwrap() }

    pub fn get_height(&self) -> i8 
        { self.mask.iter().map(|x: &Coord| x.y).max().unwrap() }

    pub fn get_init_coord(&self) -> Coord {
        let tet_height: i8 = self.get_height();
        let tet_width: i8 = self.get_width(); 
        Coord { x: (GRID_WIDTH - tet_width) / 2, 
                y:  GRID_HEIGHT - tet_height }
    }

}

