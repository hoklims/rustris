use crate::gamecore::tetronimos::{ Coord, Tetronimo, TetronimoType, TetronimoTypeIter };
use std::vec::Vec;
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

        let grid: Box<[[Option<Color>; 10]; 20]> = Box::new([[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize]);

        GameGrid { 
            tetronimos: tetronimos_vec,
            current_tetronimo: starting_tet,
            grid: grid,
            tet_coord: tet_init_coord
        }
        
    }
    fn renew_current_tetronimo(&mut self) -> () {
        self.current_tetronimo = self.tetronimos.choose().unwrap().clone();
        self.tet_coord = self.current_tetronimo.get_init_coord();
    }

    fn fix_current_tetromino(&mut self) -> () {
        let tet_mask: &[Coord; 4] = self.current_tetronimo.get_mask();
        for coord in tet_mask {
            self.grid[coord.x as usize][coord.y as usize] = Some(self.current_tetronimo.color);
        }
        self.renew_current_tetronimo();
    }

    fn forced_down_move(&mut self) -> () {
        if self.is_move_legal(Coord { x: 0, y: -1 }) 
            { self.fix_current_tetromino(); }
        else 
            { self.tet_coord += Coord { x: 0, y: -1 }; }
    }
    fn is_move_legal(&self, coord_change: Coord) -> bool { 
        //checks if tetronimo still fits in grid and does not collide with other boxes
        let mut new_coords = self.current_tetronimo.get_mask().iter()
                                         .map(|x: &Coord| *x + self.tet_coord + coord_change);
        // first, check if coords are valid
        if new_coords.all(|x: Coord| { match x.x
                                        { 0..=10 => true, _ => false }}
                                                &&
                                     { match x.y 
                                        { 0..=20 => true, _ => false}})
            // check for overlap, made in a second step to avoid converting x or y
            // to usize with negative values                                  
            { new_coords.all(|x: Coord| { match self.grid[x.x as usize][x.y as usize] 
                                            { Some(_) => false, _ => true }}) }
        
        else { false }
    }

    fn move_tet(&mut self, coord_change: Coord) -> () {
        if self.is_move_legal(coord_change) {
            self.tet_coord += coord_change ;
        }
    }

}