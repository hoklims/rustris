use crate::gamecore::game_grid::GameGrid;
use core::panic;
use std::time::{ Duration, SystemTime };

struct GameGridManager {
    score: usize,
    speed: Duration,
    level: usize
}

impl GameGridManager {

    fn manage_tet_fall(&self, grid: &mut GameGrid, last_update_time: SystemTime) -> Result<usize, String> {

        if SystemTime::now().duration_since(last_update_time).unwrap() > self.speed {
            let move_result: Result<(), String> = grid.move_tet_down();
            match move_result {
                Err(error_msg) => { match error_msg.as_str() 
                                      { "cannot allocate new tetronimo" => Err("game is over".to_string()),
                                        "move is not possible" => { grid.fix_current_tetromino(); 
                                                                    grid.renew_current_tetronimo();
                                                                    Ok(grid.remove_full_lines()) }

                                        _ => panic!("{error_msg} is not handled") }}
                Ok(_) => Ok(0) 

            }
        }
        else { Ok(0)}
    }

    pub fn level_up(&mut self) -> () {
        self.level += 1;
        let ms_speed: f64 = self.speed.as_millis() as f64;
        let new_speed: u64 = (ms_speed * 0.9) as u64;
        self.speed = Duration::from_millis(new_speed)
    }
}


