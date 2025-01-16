use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Player {
    pub username: String,
    pub units: HashMap<i32, Unit>,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ArmyMove {
    pub player: Player,
    pub units: Vec<Unit>,
    pub to_location: Location,
}

pub struct RecognitionOfWar {
    pub attacker: Player,
    pub defender: Player,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Unit {
    pub id: i32,
    pub rank: UnitRank,
    pub location: Location,
}
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Deserialize)]
pub enum UnitRank {
    Infantry,
    Cavalry,
    Artillery,
}
impl UnitRank {
    pub fn get_all_ranks() -> HashMap<UnitRank, ()> {
        HashMap::from([
            (UnitRank::Infantry, ()),
            (UnitRank::Cavalry, ()),
            (UnitRank::Artillery, ()),
        ])
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Deserialize)]
pub struct Location(pub String);

impl Location {
    pub(crate) fn get_all_locations() -> HashMap<Location, ()> {
        HashMap::from([
            (Location(String::from("americas")), ()),
            (Location(String::from("europe")), ()),
            (Location(String::from("africa")), ()),
            (Location(String::from("asia")), ()),
            (Location(String::from("australia")), ()),
            (Location(String::from("antarctica")), ()),
        ])
    }
}
