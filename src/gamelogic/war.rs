use crate::gamelogic::gamedata::{RecognitionOfWar, Unit, UnitRank};
use crate::gamelogic::gamemove::get_overlapping_location;
use crate::gamelogic::gamestate::GameState;
use crate::gamelogic::war::WarOutCome::{Draw, NoUnits, NotInvolved, OpponentWon, YouWon};
use std::cmp::PartialEq;

pub enum WarOutCome {
    NotInvolved,
    NoUnits,
    OpponentWon,
    YouWon,
    Draw,
}
pub struct War {
    pub war_out_come: WarOutCome,
    pub winner: String,
    pub loser: String,
}

impl GameState {
    pub fn handle_war(&mut self, wr: &RecognitionOfWar) -> War {
        println!("\n==== War Declared ====");
        println!(
            "{} has declared war on {}!",
            wr.attacker.username, wr.defender.username
        );
        let player = self.get_player_snap();

        if player.username == wr.defender.username {
            println!("{}, you published the war.", player.username);
            return War {
                war_out_come: NotInvolved,
                winner: "".to_string(),
                loser: "".to_string(),
            };
        }

        if player.username != wr.attacker.username {
            println!("{}, you are not involved in this war.", player.username);
            return War {
                war_out_come: NotInvolved,
                winner: "".to_string(),
                loser: "".to_string(),
            };
        }

        let overlapping = match get_overlapping_location(&wr.attacker, &wr.defender) {
            Some(location) => location,
            None => {
                println!("Error! No units are in the same location. No war will be fought.");
                return War {
                    war_out_come: NoUnits,
                    winner: "".to_string(),
                    loser: "".to_string(),
                };
            }
        };

        let defender_units: Vec<Unit> = wr
            .defender
            .units
            .iter()
            .filter(|unit| unit.1.location.eq(&overlapping))
            .map(|(_, unit)| unit.clone())
            .collect();

        let attacker_units: Vec<Unit> = wr
            .attacker
            .clone()
            .units
            .iter()
            .filter(|unit| unit.1.location.eq(&overlapping))
            .map(|unit| unit.1.clone())
            .collect();

        println!("{:?}'s units:", &wr.attacker.username);
        for unit in &attacker_units {
            println!("* {:?}\n", unit.rank);
        }
        println!("{:?}'s units:", &wr.defender.username);
        for unit in &defender_units {
            println!("* {:?}\n", unit.rank);
        }

        let attack_power = self.units_to_power_level(attacker_units);
        let defend_power = self.units_to_power_level(defender_units);
        println!("Attacker has a power level of {}", &attack_power);
        println!("Defender has a power level of {}", &defend_power);
        match attack_power {
            ap if ap > defend_power => {
                println!("{} has won the war!", wr.attacker.username);
                self.remove_unit_in_location(&overlapping);

                if player.username == wr.defender.username {
                    println!("You have lost the war!");
                    println!("Your units in {:?} have been killed.", overlapping);
                    return War {
                        war_out_come: OpponentWon,
                        winner: wr.attacker.username.clone(),
                        loser: wr.defender.username.clone(),
                    };
                }
                War {
                    war_out_come: YouWon,
                    winner: wr.attacker.username.clone(),
                    loser: wr.defender.username.clone(),
                }
            }
            ap if ap < defend_power => {
                println!("{} has won the war!", wr.defender.username);
                self.remove_unit_in_location(&overlapping);

                if player.username == wr.attacker.username {
                    println!("You have lost the war!");
                    println!("Your units in {:?} have been killed.", &overlapping);
                    return War {
                        war_out_come: OpponentWon,
                        winner: wr.defender.username.clone(),
                        loser: wr.attacker.username.clone(),
                    };
                }
                War {
                    war_out_come: YouWon,
                    winner: wr.defender.username.clone(),
                    loser: wr.attacker.username.clone(),
                }
            }
            _ => {
                println!("The war ended in a draw!");
                println!("You units in {:?} have been killed.", &overlapping);
                War {
                    war_out_come: Draw,
                    winner: wr.defender.username.clone(),
                    loser: wr.attacker.username.clone(),
                }
            }
        }
    }
    fn units_to_power_level(&self, units: Vec<Unit>) -> i32 {
        let mut power = 0;

        for unit in units {
            match unit.rank {
                UnitRank::Artillery => power += 10,
                UnitRank::Cavalry => power += 5,
                UnitRank::Infantry => power += 1,
            }
        }
        power
    }
}
