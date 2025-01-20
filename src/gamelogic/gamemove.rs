use crate::gamelogic::gamedata::{ArmyMove, Location, Player, Unit, UnitRank};
use crate::gamelogic::gamestate::GameState;
pub enum MoveOutCome {
    SamePlayer,
    Safe,
    MakeWar,
}
pub fn get_overlapping_location(p1: &Player, p2: &Player) -> Option<Location> {
    for (_, u1) in p1.units.clone() {
        for (_, u2) in p2.units.clone() {
            if u1.location == u2.location {
                return Some(u1.location.clone());
            }
        }
    }
    None
}

impl GameState {
    pub fn command_status(&self) {
        if self.is_paused() {
            println!("Paused");
        } else {
            println!("Resumed");
        }
    }

    pub fn handle_move(&self, army_move: &ArmyMove) -> MoveOutCome {
        println!("-------------");
        println!("\n");
        println!("==== Move Detected ====");
        println!(
            "{:?} is moving {} to location {:?}",
            army_move.player,
            &army_move.units.len(),
            army_move.to_location
        );
        for unit in &army_move.units {
            println!("* {:?}\n", unit.rank)
        }
        let player = self.get_player_snap();

        if player.username == army_move.player.username {
            return MoveOutCome::SamePlayer;
        };

        let overlapping_location = get_overlapping_location(&player, &army_move.player);

        match overlapping_location {
            Some(location) => {
                println!(
                    "You have overlapping units in {:?}! You are at war with {:?}",
                    location, &army_move.player
                );
                MoveOutCome::MakeWar
            }
            None => {
                println!("You're are safe from {:?}'s units", &army_move.player);
                MoveOutCome::Safe
            }
        }
    }

    pub fn command_move(&mut self, words: Vec<String>) -> Result<(), String> {
        if self.is_paused() {
            return Err("the game is paused, you can not move units".to_string());
        }
        if words.len() < 3 {
            return Err("usage: move <location> <unitID> <unitID> <unitID> etc".to_string());
        }
        let new_location = Location(words[1].clone()); // Clone to own the location

        // Validate new_location (similar to your Go code)
        if !Location::get_all_locations().contains_key(&new_location) {
            return Err(format!("error: {} is not a valid location", new_location.0));
        }

        for id_str in &words[2..] {
            let id: i32 = id_str.parse::<i32>().map_err(|err| format!("error parsing unitID: usage: move <location> <unitID> <unitID> <unitID> etc {}", err))?;
            let mut unit = self
                .get_unit(id)
                .cloned()
                .ok_or(format!("error: unit with ID {} not found", id))?;
            unit.location = new_location.clone();
            self.update_unit(&unit);
        }
        ArmyMove {
            to_location: new_location,
            units: {
                let mut units = self.get_unit_snap();
                units.sort_by_key(|u| u.id); // Sort the units by ID
                units
            },
            player: self.get_player_snap(),
        };

        Ok(())
    }
    pub fn command_spawn(&mut self, words: Vec<String>) -> Result<(), String> {
        if words.len() < 3 {
            return Err(" usage: spawn <location> <rank>".to_string());
        };
        let spawn_location = Location(words[1].clone());

        // Validate new_location (similar to your Go code)
        if !Location::get_all_locations().contains_key(&spawn_location) {
            return Err(format!(
                "error: {} is not a valid location",
                spawn_location.0
            ));
        }

        let spawn_rank = match words[2].to_lowercase().as_str() {
            "infantry" => UnitRank::Infantry,
            "cavalry" => UnitRank::Cavalry,
            "artillery" => UnitRank::Artillery,
            _ => {
                return Err(
                    "Invalid unit rank string \n  usage: spawn <location> <rank>".to_string(),
                )
            } // Or handle the error differently
        };
        let next_id = self.get_unit_snap().len();
        let unit = Unit {
            id: next_id as i32,
            location: spawn_location,
            rank: spawn_rank,
        };
        let _ = self.add_unit(unit);
        Ok(())
    }
}
