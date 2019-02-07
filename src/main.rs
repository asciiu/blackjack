use std::io;


fn play_again() -> bool {
    println!("Play again? [y|n]");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // trim newline from input
    input = String::from(input.trim());

    match input.as_ref() {
        "y" => true,
        _ => false
    }
}

fn main() {
    let mut player_continue: bool = true;

    while player_continue {
        println!("Enter a bankroll Grin amount (e.g. 1000)");

        let mut roll = String::new();

        io::stdin()
            .read_line(&mut roll)
            .expect("Failed to read line");

        // trim trailing new line
        roll = String::from(roll.trim());

        let balance: i32 = match roll.parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("balance: {}", balance);

        player_continue = play_again();
    }
}