use crate::gamecore::tetronimos::{ Coord, Tetronimo, TetronimoType, TetronimoTypeIter };
use std::{usize, vec::Vec};
use macroquad::{color::Color, rand::ChooseRandom};
use strum::IntoEnumIterator;
use std::boxed::Box;

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

type Grid = Box<[[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize]>;

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
    fn renew_current_tetronimo(&mut self) -> Result<(), String> {
        self.current_tetronimo = self.tetronimos.choose().unwrap().clone();
        self.tet_coord = self.current_tetronimo.get_init_coord();
        
        if !Self::is_move_and_mask_legal(Coord { x: 0, y: 0 },
                                         &self.current_tetronimo.mask,
                                         self.tet_coord,
                                         &self.grid)?
            { Err(String::from("cannot allocate new tetronimo")) }
        else { Ok(()) }
    }

    fn fix_current_tetromino(&mut self) -> () {
        let tet_coord = self.current_tetronimo.mask.iter().map(|x| *x + self.tet_coord);
        for coord in tet_coord {
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

    fn is_move_and_mask_legal(coord_change: Coord, tet_mask: &[Coord; 4], tet_coord: Coord, grid: &Grid) -> Result<bool, String> { 
        //checks if tetronimo still fits in grid and does not collide with other boxes
        let mut new_coords = tet_mask.iter().map(|x: &Coord| *x + tet_coord + coord_change);
        // first, check if coords are valid
        if new_coords.all(|x: Coord| { Self::is_coord_in_grid(x) })
            // check for overlap, made in a second step to avoid converting x or y
            // to usize with negative values                                  
            { Ok(new_coords.all(|x: Coord| { grid[x.x as usize][x.y as usize].is_none() })) }
        
        else { Err(String::from("tetronimo has coord outside grid")) }
    }

    fn move_tet(&mut self, coord_change: Coord) -> Result<(), String> {

        let is_legal: bool = Self::is_move_and_mask_legal(coord_change,
                                                          &self.current_tetronimo.mask,
                                                          self.tet_coord,
                                                          &self.grid)?;
        if is_legal {
            self.tet_coord += coord_change ;
            Ok(())
        }
        else { Err(String::from("move is not possible")) }
    }

    fn can_tet_change(&self) -> Result<bool, String> {
        Self::is_move_and_mask_legal(Coord { x: 0, y: 0 }, 
                                     &self.current_tetronimo.mask,
                                     self.tet_coord,
                                     &self.grid)
    }

    fn remove_line(&mut self, line_nb: usize) -> () {
        // replace each line by its up neighbor, then clears top
        for line_idx in line_nb..(GRID_HEIGHT - 1) as usize {
            self.grid[line_idx] = self.grid[line_idx + 1];
        }
        self.grid[(GRID_HEIGHT - 1) as usize] = [None; GRID_WIDTH as usize]
    }

    fn remove_full_lines(&mut self) -> () {

        let mut line_nb: usize = 0;

        while line_nb < (GRID_HEIGHT - 1) as usize {

            if self.grid[line_nb].into_iter().all(|x: Option<Color>| x.is_some()) {
                self.remove_line(line_nb);
            }
            else { line_nb += 1 }
        }
    }

}