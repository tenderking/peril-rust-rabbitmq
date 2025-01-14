use crate::gamelogic::gamedata::{ArmyMove, Location, Unit, UnitRank};
use crate::gamelogic::gamestate::GameState;

impl GameState {
    pub fn command_status(game_state: &GameState) {
        if game_state.is_paused() {
            println!("Paused");
        } else {
            println!("Resumed");
        }
    }

    pub fn command_move(game_state: &mut GameState, words: Vec<String>) -> Result<ArmyMove, String> {
        if game_state.is_paused() {
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
            let mut unit = game_state.get_unit(id).cloned().ok_or(format!("error: unit with ID {} not found", id))?;
            unit.location = new_location.clone();
            game_state.update_unit(&unit);
        }

        Ok(ArmyMove {
            to_location: new_location,
            units: {  let mut units = game_state.get_unit_snap();
                units.sort_by_key(|u| u.id); // Sort the units by ID
                units },
            player: game_state.get_player_snap(),
        })
    }
    pub fn command_spawn(game_state: &mut GameState, words: Vec<String>) -> Result<(), String> {
        if words.len() < 3 {
            return Err(" usage: spawn <location> <rank>".to_string());
        };
        let spawn_location = Location(words[1].clone());

        // Validate new_location (similar to your Go code)
        if !Location::get_all_locations().contains_key(&spawn_location) {
            return Err(format!("error: {} is not a valid location", spawn_location.0));}

            let spawn_rank= match words[2].to_lowercase().as_str() {
            "infantry"  => UnitRank::Infantry,
            "cavalry" => UnitRank::Cavalry,
            "artillery" => UnitRank::Artillery,
            _ => panic!("Invalid unit rank string"), // Or handle the error differently
        };
        let next_id = game_state.get_unit_snap().len()+1;
        let unit = Unit{
            id: next_id as i32,
            location: spawn_location,
            rank: spawn_rank
        };
        let _= game_state.add_unit(unit);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::gamelogic::gamedata::{ArmyMove, Location, Player, Unit, UnitRank};
    use crate::gamelogic::gamestate::GameState;

    #[test]
    fn command_move_test() {
        let mut gs = GameState::new("tester");
        gs.player.units.insert(
            1,
            Unit {
                id: 1,
                rank: UnitRank::Infantry,
                location: Location(String::from("initial_location")),
            },
        );
        gs.player.units.insert(
            2,
            Unit {
                id: 2,
                rank: UnitRank::Cavalry,
                location: Location(String::from("initial_location")),
            },
        );
        let command = Vec::from(
            [
                String::from("move"),
                String::from("asia"),
                String::from("1"),
                String::from("2"),
            ]
        );
        let result = GameState::command_move(&mut gs, command);

        // Assert that the result is an ArmyMove with the expected values
        assert_eq!(

            result,
            Ok(ArmyMove {
                to_location: Location(String::from("asia")),
                units: vec![
                    Unit {
                        id: 1,
                        rank: UnitRank::Infantry, // Replace with actual rank
                        location: Location(String::from("asia")),
                    },
                    Unit {
                        id: 2,
                        rank: UnitRank::Cavalry, // Replace with actual rank
                        location: Location(String::from("asia")),
                    },
                ],
                player: Player {
                    username: "tester".to_string(),
                    units: HashMap::from([
                        (
                            1,
                            Unit {
                                id: 1,
                                rank: UnitRank::Infantry, // Replace with actual rank
                                location: Location(String::from("asia")),
                            }
                        ),
                        (
                            2,
                            Unit {
                                id: 2,
                                rank: UnitRank::Cavalry, // Replace with actual rank
                                location: Location(String::from("asia")),
                            }
                        ),
                    ]),
                }
            })
        );
    }
    #[test]
    fn command_spawn_test() {
        let mut gs = GameState::new("tester");
        let command = Vec::from(
            [
                String::from("spawn"),
                String::from("asia"),
                String::from("infantry"),
            ]
        );
        GameState::command_spawn(&mut gs, command).expect("TODO: panic message");
        let result = gs.get_unit_snap();
        assert_eq!(
            result,
             vec![
                    Unit {
                        id: 1,
                        rank: UnitRank::Infantry, // Replace with actual rank
                        location: Location(String::from("asia")),
                    },
                ]
        );
    }

}