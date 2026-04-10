use crate::gamecore::tetronimos::{ Coord, Tetronimo, TetronimoType, TetronimoTypeIter };
use std::{usize, vec::Vec};
use macroquad::{color::Color, rand::ChooseRandom};
use strum::IntoEnumIterator;
use std::boxed::Box;

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

type Grid = Box<[[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize]>;

#[derive(PartialEq, Eq)]
pub enum GridError { CannotAllocateNewTet, ImpossibleMove, TetOutsideGrid }

pub struct GameGrid {

    tetronimos: Vec<Tetronimo>,
    current_tetronimo: Tetronimo,
    grid: Grid,
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

        let grid: Grid = Box::new([[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize]);

        GameGrid { 
            tetronimos: tetronimos_vec,
            current_tetronimo: starting_tet,
            grid: grid,
            tet_coord: tet_init_coord
        }
    }

    pub fn renew_current_tetronimo(&mut self) -> Result<(), GridError> {
        self.current_tetronimo = self.tetronimos.choose().unwrap().clone();
        self.tet_coord = self.current_tetronimo.get_init_coord();
        
        if !self.is_move_and_mask_legal(Coord { x: 0, y: 0 },
                                        &self.current_tetronimo.mask)?
            { Err(GridError::CannotAllocateNewTet) }

        else { Ok(()) }
    }

    pub fn fix_current_tetromino(&mut self) -> () {
        let tet_coord = self.current_tetronimo.mask.iter().map(|x: &Coord| *x + self.tet_coord);
        for coord in tet_coord {
            self.grid[coord.y as usize][coord.x as usize] = Some(self.current_tetronimo.color);
        }
    }

    fn is_coord_in_grid(coord: Coord) -> bool {
        match coord {
            Coord { x: 0..GRID_WIDTH, y: 0..GRID_HEIGHT } => true,
            _ => false
        }
    } 

    fn is_move_and_mask_legal(&self, coord_change: Coord, tet_mask: &[Coord; 4]) -> Result<bool, GridError> { 
        //checks if tetronimo still fits in grid and does not collide with other boxes
        let mut new_coords = tet_mask.iter().map(|x: &Coord| x + &self.tet_coord + coord_change);
        // first, check if coords are valid
        if new_coords.all(|x: Coord| { Self::is_coord_in_grid(x) })
            // check for overlap, made in a second step to avoid converting x or y
            // to usize with negative values                                  
            { Ok(new_coords.all(|x: Coord| { self.grid[x.y as usize][x.x as usize].is_none() })) }
        
        else { Err(GridError::TetOutsideGrid) }
    }

    fn move_tet(&mut self, coord_change: Coord) -> Result<(), GridError> {

        let is_legal: bool = self.is_move_and_mask_legal(coord_change,
                                                         &self.current_tetronimo.mask)?;

        match is_legal {
            true => { self.tet_coord += coord_change ;
                      Ok(()) }
            false => Err(GridError::ImpossibleMove)
        }
    }

    pub fn move_tet_down(&mut self) -> Result<usize, GridError> {

        let move_result: Result<(), GridError> = self.move_tet(Coord { x: 0, y: -1 });

        if let Err(error) = move_result && (error == GridError::ImpossibleMove)
            { self.fix_current_tetromino();
              self.renew_current_tetronimo()?;
              Ok(self.remove_full_lines()) }

        else {Ok(0)}         
    }

    pub fn move_tet_left(&mut self) -> Result<(), GridError> {
        self.move_tet(Coord { x: -1, y: 0 })
    }

    pub fn move_tet_right(&mut self) -> Result<(), GridError> {
        self.move_tet(Coord { x: 1, y: 0 })
    }

    fn can_tet_change(&self) -> Result<bool, GridError> {
        self.is_move_and_mask_legal(Coord { x: 0, y: 0 }, 
                                    &self.current_tetronimo.next_mask)
    }

    fn update_tet_mask(&mut self) -> Result<(), GridError> {

        if self.can_tet_change()? { self.current_tetronimo.update_mask_and_next_one(); }
        Ok(())
    }

    fn remove_line(&mut self, line_nb: usize) -> () {
        // replace each line by its up neighbor, then clears top
        for line_idx in line_nb..(GRID_HEIGHT - 1) as usize {
            self.grid[line_idx] = self.grid[line_idx + 1];
        }
        self.grid[(GRID_HEIGHT - 1) as usize] = [None; GRID_WIDTH as usize]
    }

    pub fn remove_full_lines(&mut self) -> usize {

        let mut line_nb: usize = 0;

        let mut nb_removed_lines: usize = 0;

        while line_nb < (GRID_HEIGHT - 1) as usize {

            if self.grid[line_nb].into_iter().all(|x: Option<Color>| x.is_some()) {
                self.remove_line(line_nb);
                nb_removed_lines += 1;
            }
            else { line_nb += 1 }
        }
        nb_removed_lines
    }

    fn dump_tet(&mut self) -> Result<usize, GridError> {

        let mut max_possible_down_move: Coord = Coord { x: 0, y: 0 };

        loop { 
               let new_coord: Coord = max_possible_down_move + Coord{ x: 0, y: -1 };
               if self.is_move_and_mask_legal(new_coord,
                                              &self.current_tetronimo.mask)?
                    { max_possible_down_move = new_coord; }
               else { break }
        }

        self.move_tet(max_possible_down_move)?;
        self.fix_current_tetromino();
        self.renew_current_tetronimo();
        let score: usize = self.remove_full_lines();
        
        Ok(score)
    }

}