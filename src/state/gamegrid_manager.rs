use macroquad::input::{ is_key_pressed, KeyCode };

use crate::gamecore::game_grid::{ GameGrid, GridError };
use std::time::{ Duration, SystemTime };

const TIME_DECAY: f32 = 0.9;
const BASE_SCORE_MULTIPLIER: usize = 3;

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
    ChangeMask,
    MoveLeft,
    MoveRight,
    Drop
}

pub struct GameGridManager {
    pub score: usize,
    speed: Duration,
    pub level: usize,
    last_forced_move_down: SystemTime,
    last_action: Option<Action>,
}

impl GameGridManager {

    pub fn new() -> GameGridManager { 

        GameGridManager{ score: 0,
                         speed: Duration::from_millis(1200), 
                         level: 1 , 
                         last_forced_move_down: SystemTime::now(),
                         last_action: None } 
    }

    fn manage_tet_fall(&mut self, grid: &mut GameGrid) -> Result<usize, GridError> {

        if SystemTime::now().duration_since(self.last_forced_move_down).unwrap() > self.speed 
            { self.last_forced_move_down = SystemTime::now(); grid.move_tet_down() }
        else { Ok(0) }
    }

    fn level_up(&mut self) -> () {

        self.level += 1;
        let ms_speed: f64 = self.speed.as_millis() as f64;
        let new_speed: u64 = (ms_speed * 0.9) as u64;
        self.speed = Duration::from_millis(new_speed);

    }

    fn run_action(&mut self, grid: &mut GameGrid, action: Action) -> Result<usize, GridError> {

        match action  {
                Action::MoveLeft   => { grid.move_tet_left()?; Ok(0) },
                Action::MoveRight  => { grid.move_tet_right()?; Ok(0) },
                Action::ChangeMask => { grid.update_tet_mask()?; Ok(0) }
                Action::Drop       => { Ok(grid.dump_tet()?) }
            }
    }

    fn run_game_iter(&mut self, grid: &mut GameGrid, action: Option<Action>) -> Result<(), GridError> {

        let score: usize = self.manage_tet_fall(grid)?;
        self.score += Self::scale_score(self.level, score);

        if self.score / self.level > 100 { self.level_up(); }

        match action {
            Some(player_action) => { let action_score: usize = self.run_action(grid, player_action)?;
                                             self.score += Self::scale_score(self.level, action_score);
                                             self.last_action = Some(player_action); }
            None => { self.last_action = None }
        }

        Ok(())
    }

    pub fn get_and_apply_player_input(&mut self, grid: &mut GameGrid) -> Result<(), GridError> {

        match (is_key_pressed(KeyCode::Up),
               is_key_pressed(KeyCode::Left),
               is_key_pressed(KeyCode::Right),
               is_key_pressed(KeyCode::Down)) {
            (true, ..) => self.run_game_iter(grid, Some(Action::ChangeMask)),
            (_, true, ..) => self.run_game_iter(grid, Some(Action::MoveLeft)),
            (.., true, _) => self.run_game_iter(grid, Some(Action::MoveRight)),
            (.., true) => self.run_game_iter(grid, Some(Action::Drop)),
            _ => self.run_game_iter(grid, None)
        }
    }

    fn scale_score(level: usize, score: usize) -> usize {
        ( (2.0 - TIME_DECAY).powf(level as f32) * score as f32 ) as usize * BASE_SCORE_MULTIPLIER
    }    
}


