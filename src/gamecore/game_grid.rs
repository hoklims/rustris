use crate::gamecore::tetrominos::{ Coord, Tetromino, TetrominoType, TetrominoTypeIter };
use std::{ usize, vec::Vec };
use macroquad::{ color::Color, 
                 rand::ChooseRandom };
use strum::IntoEnumIterator;
use std::boxed::Box;

pub const GRID_HEIGHT: i8 = 20;
pub const GRID_WIDTH: i8 = 10;

type Grid = Box<[[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize]>;

#[derive(PartialEq, Eq, Debug)]
pub enum GridError { 
    CannotAllocateNewTet,
    ImpossibleMove,
    TetOutsideGrid,
    TetWentThroughFloor
}

pub struct GameGrid {
    tetrominos: Vec<Tetromino>,
    pub current_tetromino: Option<Tetromino>,
    pub grid: Grid,
    pub tet_coord: Coord
}

impl GameGrid {

    pub fn new() -> GameGrid {

        let tet_type_iter: TetrominoTypeIter = TetrominoType::iter();
        let mut tetrominos_vec: Vec<Tetromino> = Vec::with_capacity(tet_type_iter.len());

        for tet_type in tet_type_iter {
            tetrominos_vec.push(Tetromino::new(tet_type));
        }

        let starting_tet: Tetromino = tetrominos_vec.choose().unwrap().clone();
        let tet_init_coord : Coord = starting_tet.get_init_coord();

        let grid: Grid = Box::new([[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize]);

        GameGrid { 
            tetrominos: tetrominos_vec,
            current_tetromino: Some(starting_tet),
            grid: grid,
            tet_coord: tet_init_coord
        }
    }

    pub fn renew_current_tetromino(&mut self) -> Result<(), GridError> {
        self.current_tetromino = Some(self.tetrominos.choose().unwrap().clone());
        self.tet_coord = self.current_tetromino.as_ref().unwrap().get_init_coord();
        
        if let Err(_) = self.is_move_and_mask_legal(Coord { x: 0, y: 0 },
                                                    &self.current_tetromino.as_ref().unwrap().mask)
            { self.current_tetromino = None; Err(GridError::CannotAllocateNewTet) }

        else { Ok(()) }
    }

    fn fix_current_tetromino(&mut self) -> () {
        let tet_coord = self.current_tetromino.as_ref().unwrap().mask.iter()
                                                        .map(|x: &Coord| *x + self.tet_coord );
        for coord in tet_coord {
            self.grid[coord.y as usize][coord.x as usize] = Some(self.current_tetromino.as_ref().unwrap().color);
        }
    }

    fn went_through_floor(coord: &Coord) -> bool {
        match coord {
            Coord { x: 0..GRID_WIDTH, y: -1 } => true,
            _ => false
        }
    }

    fn is_coord_in_grid(coord: &Coord) -> bool {
        match coord {
            Coord { x: 0..GRID_WIDTH, y: 0..GRID_HEIGHT } => true,
            _ => false
        }
    } 

    fn is_move_and_mask_legal(&self, coord_change: Coord, 
                              tet_mask: &[Coord; 4]) -> Result<bool, GridError> { 
        //checks if tetromino still fits in grid and does not collide with other boxes
        let new_coords: [Coord; 4] = tet_mask.map(|x: Coord| x + self.tet_coord + coord_change );
        // first, check if coords are valid

        if new_coords.iter().all(|x: &Coord| Self::is_coord_in_grid(x) )
            // check for overlap, made in a second step to avoid converting x or y
            // to usize with negative values
            {
              if new_coords.iter().all(|x: &Coord| self.grid[x.y as usize][x.x as usize].is_none() )
                  { Ok(true) }
              else 
                  { Err(GridError::ImpossibleMove) }   
            }

        else if new_coords.iter().any(|x: &Coord| Self::went_through_floor(x) ) 
            { Err(GridError::TetWentThroughFloor)  }
        
        else { Err(GridError::TetOutsideGrid) }
    }

    fn move_tet(&mut self, coord_change: Coord) -> Result<(), GridError> {

        let is_legal_res: Result<bool, GridError> = self.is_move_and_mask_legal(
            coord_change,
            &self.current_tetromino.as_ref().unwrap().mask);

        match is_legal_res {
            Ok(res) if res => { self.tet_coord += coord_change; Ok(()) },
            Ok(res) if !res => Ok(()),
            Err(error) => Err(error),
            _ => panic!("legal move result is neither a bool or an error")

        }
    }

    pub fn move_tet_down(&mut self) -> Result<usize, GridError> {

        if self.current_tetromino.is_none()  { return Ok(0); }

        let move_result: Result<(), GridError> = self.move_tet(Coord { x: 0, y: -1 });
        if let Err(error) = move_result && 
           (error == GridError::ImpossibleMove || error == GridError::TetWentThroughFloor)
                { self.fix_current_tetromino();
                  self.renew_current_tetromino()?;
                  Ok(self.remove_full_lines()) }

        else { Ok(0) }         
    }

    pub fn move_tet_left(&mut self) -> Result<(), GridError> {

        if self.current_tetromino.is_none()  { return Ok(()); }

        let move_result: Result<(), GridError> = self.move_tet(Coord { x: -1, y: 0 });
        match move_result {
            Err(result) => { if result == GridError::ImpossibleMove ||
                                           result == GridError::TetOutsideGrid { Ok(()) }
                                        else { Err(result)} }
            Ok(_) => Ok(())
        }
    }

    pub fn move_tet_right(&mut self) -> Result<(), GridError> {

        if self.current_tetromino.is_none()  { return Ok(()); }

        let move_result: Result<(), GridError> = self.move_tet(Coord { x: 1, y: 0 });
        match move_result {
            Err(result) => { if result == GridError::ImpossibleMove ||
                                           result == GridError::TetOutsideGrid { Ok(()) }
                                        else { Err(result)} }
            Ok(_) => Ok(())
        }
    }

    fn can_tet_change(&self) -> Result<bool, GridError> {

        if self.current_tetromino.is_none() { return Ok(false); }
        
        self.is_move_and_mask_legal(
            Coord { x: 0, y: 0 }, 
            &self.current_tetromino.as_ref().unwrap().next_mask
        )
    }

    pub fn update_tet_mask(&mut self) -> Result<(), GridError> {
        if let Ok(possible) = self.can_tet_change() && possible 
            { self.current_tetromino.as_mut().unwrap().update_mask_and_next_one(); }

        Ok(())
    }

    fn remove_line(&mut self, line_nb: usize) -> () {
        // replace each line by its up neighbor, then clears top
        for line_idx in line_nb..(GRID_HEIGHT - 1) as usize 
            { self.grid[line_idx] = self.grid[line_idx + 1]; }
        self.grid[(GRID_HEIGHT - 1) as usize] = [None; GRID_WIDTH as usize]
    }

     fn remove_full_lines(&mut self) -> usize {

        let mut line_nb: usize = 0;
        let mut nb_removed_lines: usize = 0;

        while line_nb < GRID_HEIGHT as usize {

            if !self.grid[line_nb].iter().any(|x: &Option<Color>| x.is_none() ) {
                self.remove_line(line_nb);
                nb_removed_lines += 1;
            }
            else { line_nb += 1 }
        }
        nb_removed_lines
    }

    pub fn dump_tet(&mut self) -> Result<usize, GridError> {

        if self.current_tetromino.is_none() { return Ok(0); }

        let mut max_possible_down_move: Coord = Coord { x: 0, y: 0 };

        loop { 
            let new_coord: Coord = max_possible_down_move + Coord{ x: 0, y: -1 };
            if let Ok(is_legal) = self.is_move_and_mask_legal(
                    new_coord, 
                    self.current_tetromino.as_ref().unwrap().mask)
                && is_legal
                { max_possible_down_move = new_coord; }
            else { break; }
        }

        self.move_tet(max_possible_down_move)?;
        self.fix_current_tetromino();
        let score: usize = self.remove_full_lines();
        self.renew_current_tetromino()?;
        
        Ok(score)
    }

}