use std::io::{self};
use rand::prelude::*;
pub fn  print_client_help() {
    println!("Possible commands:");
    println!("* move <location> <unitID> <unitID> <unitID>...");
    println!("    example:");
    println!("    move asia 1");
    println!("* spawn <location> <rank>");
    println!("    example:");
    println!("    spawn europe infantry");
    println!("* status");
    println!("* spam <n>");
    println!("    example:");
    println!("    spam 5");
    println!("* quit");
    println!("* help");
}
pub fn print_server_help() {
    println!("Possible commands:");
    println!("* pause");
    println!("* resume");
    println!("* quit");
    println!("* help");
}
fn get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn client_welcome() -> Result<String, &'static str> {
    println!("Welcome to the Peril client");
    println!("Please enter your username:");
    let words = get_input();
    if words.is_empty() {
        return Err("Please enter a username");
    }
    let username = words[0].trim();
    Ok(username.to_string())
}
fn get_malicious_log() -> String {
    let possible_logs = vec![
        "Never interrupt your enemy when he is making a mistake.".to_string(),
        "The hardest thing of all for a soldier is to retreat.".to_string(),
        "A soldier will fight long and hard for a bit of colored ribbon.".to_string(),
        "It is well that war is so terrible, otherwise we should grow too fond of it.".to_string(),
        "The art of war is simple enough. Find out where your enemy is. Get at him as soon as you can. Strike him as hard as you can, and keep moving on.".to_string(),
        "All warfare is based on deception.".to_string(),
    ];

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..possible_logs.len());
    possible_logs[random_index].clone()
}
pub fn print_quit() {
    println!("I hate this game! (╯°□°)╯︵ ┻━┻")
}
