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
#[cfg(test)]
mod tests {

    use crate::gamelogic::gamedata::{Location, Unit, UnitRank};
    use crate::gamelogic::gamestate::GameState;

    #[test]
    fn update_unit_test() {
        let mut gs = GameState::new("tester");
        let initial_location = Location(String::from("initial_location"));

        // Add a unit to the GameState
        gs.player.units.insert(
            1,
            Unit {
                id: 1,
                rank: UnitRank::Infantry,
                location: initial_location.clone(),
            },
        );

        // Create an updated unit
        let updated_unit = Unit {
            id: 1,
            rank: UnitRank::Infantry,
            location: Location(String::from("new_location")),
        };

        // Call the update_unit method
        gs.update_unit(&updated_unit);

        // Get the unit from the GameState and assert its location is updated
        let fetched_unit = gs.get_unit(1).unwrap(); // Assuming get_unit returns Option<&Unit>
        assert_eq!(
            fetched_unit.location,
            Location(String::from("new_location"))
        );
    }
}
