use crate::gamecore::game_grid::{ GameGrid, GridError};
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
    last_update: SystemTime,
    last_action_time: SystemTime,
    last_action: Action,
    action_delay: Duration
}

impl GameGridManager {

    fn new() -> GameGridManager { 

        GameGridManager{ score: 0,
                         speed: Duration::from_millis(1200), 
                         level: 1 , 
                         last_update: SystemTime::now(),
                         last_action_time: SystemTime::now(),
                         last_action: Action::Drop } 
    }

    fn manage_tet_fall(&mut self, grid: &mut GameGrid) -> Result<usize, GridError> {

        if SystemTime::now().duration_since(self.last_update).unwrap() > self.speed 
            { self.last_update = SystemTime::now(); grid.move_tet_down() }
        else { Ok(0) }
    }

    pub fn level_up(&mut self) -> () {

        self.level += 1;
        let ms_speed: f64 = self.speed.as_millis() as f64;
        let new_speed: u64 = (ms_speed * 0.9) as u64;
        self.speed = Duration::from_millis(new_speed)

    }

    fn run_action(&mut self, grid: &mut GameGrid, action: Action) -> Result<usize, GridError> {

        match action  {
                Action::MoveLeft   => { grid.move_tet_left()?; Ok(0) },
                Action::MoveRight  => { grid.move_tet_right()?; Ok(0) },
                Action::ChangeMask => { grid.update_tet_mask()?; Ok(0) }
                Action::Drop       => { Ok(grid.dump_tet()?) }
            }
    }

    fn must_block_action(&self, action: Action) -> bool {

        let is_same_action: bool = action == self.last_action;
        let elapsed_time_since_last_action: Duration = SystemTime::now().duration_since(self.last_update).unwrap();
        let is_delay_exceeded: bool = elapsed_time_since_last_action > self.action_delay;
        let is_action_change_mask: bool = action == Action::ChangeMask;

            is_same_action && is_delay_exceeded && !is_action_change_mask        
    }

    fn run_game_iter(&mut self, grid: &mut GameGrid, action: Option<Action>) -> Result<(), GridError> {

        let score: usize = self.manage_tet_fall(grid)?;
        self.score += score;

        if self.score / self.level > 100 { self.level_up(); }

        if let Some(player_action) = action && !self.must_block_action(player_action)
            { self.score += self.run_action(grid, player_action)?; }

        Ok(())
    }
}


