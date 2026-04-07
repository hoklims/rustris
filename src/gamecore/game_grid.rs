use crate::gamecore::tetronimos::{ Coord, Tetronimo, TetronimoType, TetronimoTypeIter };
use std::{usize, vec::Vec};
use macroquad::{color::Color, rand::ChooseRandom};
use strum::IntoEnumIterator;
use std::boxed::Box;

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

pub struct GameGrid {

    tetronimos: Vec<Tetronimo>,
    current_tetronimo: Tetronimo,
    grid: Box<[[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize]>,
    tet_coord: Coord

}

impl GameGrid {

    pub fn new() -> GameGrid {

        let tet_type_iter: TetronimoTypeIter = TetronimoType::iter();
        let mut tetronimos_vec: Vec<Tetronimo> = Vec::with_capacity(tet_type_iter.len());

        for tet_type in tet_type_iter {
            tetronimos_vec.push(Tetronimo::new(tet_type));
        }

        let starting_tet: Tetronimo = tetronimos_vec.choose().unwrap().clone();
        let tet_init_coord : Coord = starting_tet.get_init_coord();

        let grid: Box<[[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize]> = 
                  Box::new([[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize]);

        GameGrid { 
            tetronimos: tetronimos_vec,
            current_tetronimo: starting_tet,
            grid: grid,
            tet_coord: tet_init_coord
        }
        
    }
    fn renew_current_tetronimo(&mut self) -> Result<(), &str> {
        self.current_tetronimo = self.tetronimos.choose().unwrap().clone();
        self.tet_coord = self.current_tetronimo.get_init_coord();
        
        if !self.is_move_and_mask_legal(Coord { x: 0, y: 0 },
                                       self.current_tetronimo.get_mask())
            { Err("cannot allocate new tetronimo") }
        else { Ok(()) }
    }

    fn fix_current_tetromino(&mut self) -> () {
        let tet_mask: &[Coord; 4] = self.current_tetronimo.get_mask();
        for coord in tet_mask {
            self.grid[coord.x as usize][coord.y as usize] = Some(self.current_tetronimo.color);
        }
        //self.renew_current_tetronimo();
    }

    fn is_coord_in_grid(coord: Coord) -> bool {
        match coord {
            Coord { x: 0..GRID_WIDTH, y: 0..GRID_HEIGHT } => true,
            _ => false
        }
    } 

    fn forced_down_move(&mut self) -> () {
        if self.is_move_and_mask_legal(Coord { x: 0, y: -1 },
                                       self.current_tetronimo.get_mask()) 
            { self.tet_coord += Coord { x: 0, y: -1 }; }
        else 
            { self.fix_current_tetromino(); }
    }

    fn is_move_and_mask_legal(&self, coord_change: Coord, mask: &[Coord; 4]) -> bool { 
        //checks if tetronimo still fits in grid and does not collide with other boxes
        let mut new_coords = mask.iter().map(|x: &Coord| *x + self.tet_coord + coord_change);
        // first, check if coords are valid
        if new_coords.all(|x: Coord| { Self::is_coord_in_grid(x) })
            // check for overlap, made in a second step to avoid converting x or y
            // to usize with negative values                                  
            { new_coords.all(|x: Coord| { self.grid[x.x as usize][x.y as usize].is_none() }) }
        
        else { false }
    }

    fn move_tet(&mut self, coord_change: Coord) -> () {
        if self.is_move_and_mask_legal(coord_change, self.current_tetronimo.get_mask()) {
            self.tet_coord += coord_change ;
        }
    }

    fn can_tet_change(&self) -> bool {
        self.is_move_and_mask_legal(Coord { x: 0, y: 0 }, 
                                    self.current_tetronimo.get_next_mask())
    }

    fn remove_line(&mut self, line_nb: usize) -> () {
        // replace each line by its up neighbor, then clears top
        for line_idx in line_nb..(GRID_HEIGHT-1) as usize {
            self.grid[line_idx] = self.grid[line_idx + 1];
        }
        self.grid[GRID_HEIGHT as usize] = [None; GRID_WIDTH as usize]
    }

    fn remove_full_lines(&mut self) -> () {

        let mut line_nb: usize = 0;

        while line_nb < (GRID_HEIGHT - 1) as usize {
            if self.grid[line_nb].into_iter().all(|x| x.is_some()) {
                self.remove_line(line_nb);
            }
            else { line_nb += 1 }
        }
    }

}