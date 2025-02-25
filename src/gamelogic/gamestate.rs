use crate::gamelogic::gamedata::{Location, Player, Unit};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct GameState {
    pub player: Player,
    pub paused: bool,
    pub mu: Arc<Mutex<()>>,
}

impl GameState {
    pub fn new(username: &str) -> GameState {
        let new_game = GameState {
            player: Player {
                username: String::from(username),
                units: HashMap::new(),
            },
            paused: false,
            mu: Arc::new(Mutex::new(())),
        };
        new_game
    }

    pub fn resume_game(&mut self) {
        let _guard = self.mu.lock().unwrap(); // Acquire the lock
        self.paused = true;
    }
    pub fn pause_game(&mut self) {
        let _guard = self.mu.lock().unwrap(); // Acquire the lock
        self.paused = true;
    }
    pub fn is_paused(&self) -> bool {
        let _guard = self.mu.lock().unwrap(); // Acquire a read lock
        self.paused
    }

    pub fn add_unit(&mut self, unit: Unit) {
        let _guard = self.mu.lock().unwrap();
        self.player.units.insert(unit.id, unit);
    }

    pub fn remove_unit_in_location(&mut self, loc: &Location) {
        let _guard = self.mu.lock().unwrap();
        for (k, v) in self.player.units.clone() {
            if v.location == *loc {
                self.player.units.remove(&k);
            }
        }
    }

    pub fn update_unit(&mut self, unit: &Unit) {
        let _guard = self.mu.lock().unwrap(); // If you're using a mutex
        self.player.units.insert(unit.id, unit.clone());
    }
    pub fn get_username(&self) -> String {
        self.player.username.clone()
    }

    pub fn get_unit_snap(&self) -> Vec<Unit> {
        let _guard = self.mu.lock().unwrap();
        self.player.units.values().cloned().collect()
    }

    pub fn get_unit(&self, id: i32) -> Option<&Unit> {
        let _guard = self.mu.lock().unwrap();
        self.player.units.get(&id)
    }
    pub fn get_player_snap(&self) -> Player {
        let _guard = self.mu.lock().unwrap();
        let units = self
            .player
            .units
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();

        Player {
            username: self.player.username.clone(),
            units,
        }
    }
}
