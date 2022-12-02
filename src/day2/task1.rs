use crate::day2;

pub enum GameState {
    Rock,
    Paper,
    Scissors,
}

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

        let p2 = match vec[1] {
            "X" => GameState::Rock,
            "Y" => GameState::Paper,
            "Z" => GameState::Scissors,
            _ => GameState::Rock,
        };

        sum += get_winer(&p1, &p2);
        sum += get_point(&p2);
    }

    println!("Task 1: {}", sum);
}

pub fn get_winer(p1: &GameState, p2: &GameState) -> i32 {
    match p1 {
        GameState::Rock => match p2 {
            GameState::Rock => 3,
            GameState::Paper => 6,
            GameState::Scissors => 0,
        },
        GameState::Paper => match p2 {
            GameState::Rock => 0,
            GameState::Paper => 3,
            GameState::Scissors => 6,
        },
        GameState::Scissors => match p2 {
            GameState::Rock => 6,
            GameState::Paper => 0,
            GameState::Scissors => 3,
        },
    }
}

pub fn get_point(p1: &GameState) -> i32 {
    match p1 {
        GameState::Rock => 1,
        GameState::Paper => 2,
        GameState::Scissors => 3,
    }
}
