// standard library module uses.
use std::io;

// internal module uses.
mod choices;
use choices::Choice;

// external module uses.
use rand::Rng;

// Conclusion represents the possible endings of the rpsls match.
enum Conclusion {
    // String represents the action that was performed against the robot to win.
    Player(String),
    // String represents the action that was performed against the user to win.
    Robot(String),
    // Tie doesn't require any actions or extra parameters.
    Tie,
}

fn main() {
    // Display the choices
    print!("\t0.Rock\n\t1.Paper\n\t2.Scissors\n\t3.Lizard\n\t4.Spock\n\n");
    // Read the user's response after the choices are displayed.
    let mut user_input = String::new();
    // Read standard in and panic if any errors.
    io::stdin()
        .read_line(&mut user_input)
        .expect("reading input");
    // Cast the user_choice as an unsigned integer.
    let user_int: u32 = user_input
        .trim() // trim newline and carriage return characters.
        .parse() // parse a u32
        .expect("input could not be parsed");
    // Get a choice from the unsigned integer.
    let user_choice = Choice::from_u32(user_int).expect("invalid choice");
    println!("User chose {}", user_choice);

    // Create a new random number generator.
    let mut rng = rand::thread_rng();
    let robot_choice = Choice::from_u32(rng.gen_range(1, 5)).expect("rng invalid");
    println!("Robot chose {}", robot_choice);
    println!(); // extra newline

    // fight and match the result of whether the player won, robot won, or tied.
    // this match will become more helpful later down the line when this app
    // moves off of text.
    match fight(&user_choice, &robot_choice) {
        Conclusion::Player(action) =>
            println!("{} {} {}\n\nPlayer wins", user_choice, action, robot_choice),
        Conclusion::Robot(action) => 
            println!("{} {} {}\n\nRobot wins", robot_choice, action, user_choice),
		Conclusion::Tie =>
			println!("{} and {} tie", user_choice, robot_choice),
    }
}

fn fight(user: &Choice, robot: &Choice) -> Conclusion {
    // See if the player defeats the robot.
    if let Some(action) = user.wins_against(&robot) {
        return Conclusion::Player(action);
    } else if let Some(action) = robot.wins_against(&user) {
        // We run this if let, because we need the action of the robot defeating
        // the player.
        return Conclusion::Robot(action);
    }
    // Conclude a tie if nothing else.
    Conclusion::Tie
}
