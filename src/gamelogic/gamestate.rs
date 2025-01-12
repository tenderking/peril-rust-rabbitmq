use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::gamelogic::gamedata::{Location, Player, Unit};



pub struct  GameState {
    pub player: Player,
    pub paused: bool,
    pub mu: Arc<Mutex<()>>
}

impl GameState {
    fn new_game_state(username: String) -> GameState {
        let new_game = GameState {
            player: Player {
                username: String::from(username),
                units: HashMap::new()
            },
            paused: false,
            mu: Arc::new(Mutex::new(()))
        };
       new_game
    }

    fn resume_game(&mut self, ) {
        let _guard = self.mu.lock().unwrap(); // Acquire the lock
        self.paused = true;
    }
    fn pause_game(&mut self, ) {
        let _guard = self.mu.lock().unwrap(); // Acquire the lock
        self.paused = true;
    }
    fn is_paused(&self) -> bool {
        let _guard = self.mu.lock().unwrap(); // Acquire a read lock
        self.paused
    }

    fn add_unit(&mut self, unit: Unit) {
        let _guard = self.mu.lock().unwrap();
        self.player.units.insert(unit.id, unit);
    }

    fn remove_unit_in_location(&mut self, loc: &Location) {
        let _guard = self.mu.lock().unwrap();
        for (k, v )in self.player.units.clone() {
          if v.location == *loc {
            self.player.units.remove(&k);
            }
        }
    }

    fn update_unit(&mut self, unit: Unit) {
        let _guard = self.mu.lock().unwrap();
        self.player.units.insert(unit.id, unit);
    }
    fn get_username(&self) -> String {
       self.player.username.clone()
    }

    fn get_unit_snap(&self) -> Vec<Unit> {
        let _guard = self.mu.lock().unwrap();
        self.player.units.values().cloned().collect()
    }

    fn get_unit(&self, id: i32) -> (&Unit, bool) {
    let _guard = self.mu.lock().unwrap();

        (self.player.units.get(&id).unwrap(), true)
    }

    fn get_player_snap(&self) -> Player {
        let _guard = self.mu.lock().unwrap();
        let units = self.player.units.iter().map(|(k, v)| (*k, v.clone())).collect();

        Player {
            username: self.player.username.clone(),
            units,
        }
    }


}