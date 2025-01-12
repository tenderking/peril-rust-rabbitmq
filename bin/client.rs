use risk_rust::gamelogic::gamelogic;


fn main() {
    println!("Hello, world!");
    match gamelogic::client_welcome() {
        Ok(username) => {
            println!("Username: {:?}", username);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}