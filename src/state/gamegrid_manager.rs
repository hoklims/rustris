use crate::gamecore::game_grid::{ GameGrid, GridError };
use std::time::{ Duration, SystemTime };

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
    ChangeMask,
    MoveLeft,
    MoveRight,
    Drop
}

struct GameGridManager {
    score: usize,
    speed: Duration,
    level: usize,
    last_forced_move_down: SystemTime,
    last_action: Option<Action>,
}

impl GameGridManager {

    fn new() -> GameGridManager { 

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

    pub fn level_up(&mut self) -> () {

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
        self.score += score;

        if self.score / self.level > 100 { self.level_up(); }

        match action {
            Some(player_action) => { self.score += self.run_action(grid, player_action)?;
                                             self.last_action = Some(player_action); }
            None => { self.last_action = None }
        }

        Ok(())
    }
}


