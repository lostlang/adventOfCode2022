use crate::day6;

pub fn main() {
    let values = day6::readfiles::read();

    let mut i = 0;

    while i < values.len() - 3 {
        let word = values.get(i..i + 4).unwrap();
        let sh = shift(word);

        if sh > -1 {
            i += (sh + 1) as usize;
        } else {
            break;
        }
    }

    println!("Task 1: {}", i + 4);
}

fn shift(word: &str) -> i32 {
    let mut count = -1;

    for i in 0..word.len() {
        for j in i + 1..word.len() {
            if word.get(i..i + 1).unwrap() == word.get(j..j + 1).unwrap() {
                count = i as i32;
            }
        }
    }

    count
}
