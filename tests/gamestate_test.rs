#[cfg(test)]
mod tests {

    use risk_rust::gamelogic::gamedata::{Location, Unit, UnitRank};
    use risk_rust::gamelogic::gamestate::GameState;

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
    #[test]
    fn get_player_snap_test() {
        let mut gs = GameState::new("tester");
        let initial_location = Location(String::from("asia"));

        // Insert a unit into the player's units
        gs.player.units.insert(
            1,
            Unit {
                id: 1,
                rank: UnitRank::Infantry,
                location: initial_location.clone(),
            },
        );

        // Get the player snapshot
        let player_snap = gs.get_player_snap();

        // Verify username is the same
        assert_eq!(player_snap.username, "tester");

        // Verify unit exists in the snapshot
        assert!(player_snap.units.contains_key(&1));
        let unit = player_snap.units.get(&1).unwrap();
        assert_eq!(unit.id, 1);
        assert_eq!(unit.rank, UnitRank::Infantry);
        assert_eq!(unit.location, initial_location);
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
