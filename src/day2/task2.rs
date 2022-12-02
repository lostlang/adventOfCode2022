use crate::day2;
use crate::day2::task1::{get_point, get_winer, GameState};

pub fn main() {
    let values = day2::readfiles::read();

    let mut sum = 0;

    for v in values {
        let vec = v.split(" ").collect::<Vec<&str>>();

        let p1 = match vec[0] {
            "A" => GameState::Rock,
            "B" => GameState::Paper,
            "C" => GameState::Scissors,
            _ => GameState::Rock,
        };

        let p2 = get_taktik(&p1, vec[1]);

        sum += get_winer(&p1, &p2);
        sum += get_point(&p2);
    }

    println!("Task 2: {}", sum);
}

fn get_taktik(p1: &GameState, v2: &str) -> GameState {
    match p1 {
        GameState::Rock => match v2 {
            "X" => GameState::Scissors,
            "Y" => GameState::Rock,
            "Z" => GameState::Paper,
            _ => GameState::Rock,
        },
        GameState::Paper => match v2 {
            "X" => GameState::Rock,
            "Y" => GameState::Paper,
            "Z" => GameState::Scissors,
            _ => GameState::Paper,
        },
        GameState::Scissors => match v2 {
            "X" => GameState::Paper,
            "Y" => GameState::Scissors,
            "Z" => GameState::Rock,
            _ => GameState::Scissors,
        },
    }
}
