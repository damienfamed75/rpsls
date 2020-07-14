use std::fmt::Display;
// WinCondition ressembles what a choice wins against and follows the tuple
// pattern of: (losing_hand, action)
//
// Where "losing_hand" is the opponent's bad hand
// And where "action" is the action performed against the opponent to win.
//
// WinCondition is a fixed length of 2 because in rpsls every choice can win
// against 2 other hands.
type WinCondition<'a> = [(Choice, &'a str); 2];

// Choice is an enumeration of the possible choices when playing rpsls./
//
// derives:
// (PartialEq, Eq) allow for comparisons between Choices such as A == B
#[derive(PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Choice {
    pub fn from_u32(i: u32) -> Option<Choice> {
        match i {
            0 => Some(Choice::Rock),
            1 => Some(Choice::Paper),
            2 => Some(Choice::Scissors),
            3 => Some(Choice::Lizard),
            4 => Some(Choice::Spock),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissors => "Scissors",
            Choice::Lizard => "Lizard",
            Choice::Spock => "Spock",
        }
    }
    // wins_against returns an optional action string if this choice wins
    // against the opponent choice.
    //
    // If this choice does not win, then None is returned.
    pub fn wins_against(&self, other: &Choice) -> Option<String> {
        let self_win_conditions = self.win_condition();
        // Loop through the array of winning conditions.
        // Extract the tuple values when looping over the iterator.
        for (losing_choice, action) in self_win_conditions.iter() {
            // Dereference the losing_choice and compare it to the opponent.
            if *losing_choice == *other {
                return Some(String::from(*action));
            }
        }
        None
    }
    // win_condition returns what this choice wins against along with the action
    // performed on the choice to win.
    //
    // Winning conditions follow a tuple pattern of (losing_choice, action)
    fn win_condition<'a>(&self) -> WinCondition<'a> {
        match self {
            // [ (losing_choice, action), (losing_choice, action) ]
            Choice::Rock => [(Choice::Lizard, "crushes"), (Choice::Scissors, "crushes")],
            Choice::Paper => [(Choice::Rock, "covers"), (Choice::Spock, "disproves")],
            Choice::Scissors => [(Choice::Paper, "cuts"), (Choice::Lizard, "decapitates")],
            Choice::Lizard => [(Choice::Spock, "poisons"), (Choice::Paper, "eats")],
            Choice::Spock => [(Choice::Rock, "vaporizes"), (Choice::Scissors, "smashes")],
        }
    }
}
