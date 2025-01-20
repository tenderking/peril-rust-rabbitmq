#[cfg(test)]
mod tests {
    use risk_rust::gamelogic::gamedata::{Location, Player, Unit, UnitRank};
    use risk_rust::gamelogic::gamestate::GameState;
    use std::collections::HashMap;

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
        let command = Vec::from([
            String::from("move"),
            String::from("asia"),
            String::from("1"),
            String::from("2"),
        ]);
        GameState::command_move(&mut gs, command).expect("TODO: panic message");
        let result = gs.get_player_snap();
        // Assert that the result is an ArmyMove with the expected values

        let player = Player {
            username: "tester".to_string(),
            units: HashMap::from([
                (
                    1,
                    Unit {
                        id: 1,
                        rank: UnitRank::Infantry, // Replace with actual rank
                        location: Location(String::from("asia")),
                    },
                ),
                (
                    2,
                    Unit {
                        id: 2,
                        rank: UnitRank::Cavalry, // Replace with actual rank
                        location: Location(String::from("asia")),
                    },
                ),
            ]),
        };
        assert_eq!(result, player);
    }
    #[test]
    fn command_spawn_test() {
        let mut gs = GameState::new("tester");
        let command = Vec::from([
            String::from("spawn"),
            String::from("asia"),
            String::from("infantry"),
        ]);
        GameState::command_spawn(&mut gs, command).expect("TODO: panic message");
        let result = gs.get_unit_snap();
        assert_eq!(
            result,
            vec![Unit {
                id: 0,
                rank: UnitRank::Infantry, // Replace with actual rank
                location: Location(String::from("asia")),
            },]
        );
    }
    #[test]
    fn remove_unit_in_location_test() {
        let mut gs = GameState::new("tester");
        let loc1 = Location(String::from("loc1"));
        let loc2 = Location(String::from("loc2"));

        // Add units to different locations
        gs.player.units.insert(
            1,
            Unit {
                id: 1,
                rank: UnitRank::Infantry,
                location: loc1.clone(),
            },
        );
        gs.player.units.insert(
            2,
            Unit {
                id: 2,
                rank: UnitRank::Cavalry,
                location: loc2.clone(),
            },
        );
        gs.player.units.insert(
            3,
            Unit {
                id: 3,
                rank: UnitRank::Artillery,
                location: loc1.clone(),
            },
        );

        // Remove units in loc1
        gs.remove_unit_in_location(&loc1);

        // Check if units in loc1 are removed
        assert!(gs.player.units.get(&1).is_none());
        assert!(gs.player.units.get(&3).is_none());

        // Check if unit in loc2 remains
        assert!(gs.player.units.get(&2).is_some());
        assert_eq!(gs.player.units.get(&2).unwrap().location, loc2);
    }
}
