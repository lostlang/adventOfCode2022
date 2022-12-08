use crate::day8;

pub fn main() {
    let values = day8::readfiles::read();

    let mut max = 0;

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            let val = deep_search(&values, i, j);
            if val > max {
                max = val;
                println!("{} {} {}", i, j, max);
            }
        }
    }

    println!("Task 2: {}", max);
}

fn deep_search(values: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let count = max_left(values, i, j)
        * max_right(values, i, j)
        * max_up(values, i, j)
        * max_down(values, i, j);

    count
}

fn max_left(values: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut max = 1;

    for k in (1..j).rev() {
        if values[i][k] < values[i][j] {
            max += 1;
        } else {
            break;
        }
    }

    max
}

fn max_right(values: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut max = 1;

    for k in j + 1..values[i].len() - 1 {
        if values[i][k] < values[i][j] {
            max += 1;
        } else {
            break;
        }
    }

    max
}

fn max_up(values: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut max = 1;

    for k in (1..i).rev() {
        if values[k][j] < values[i][j] {
            max += 1;
        } else {
            break;
        }
    }

    max
}

fn max_down(values: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut max = 1;

    for k in i + 1..values.len() - 1 {
        if values[k][j] < values[i][j] {
            max += 1;
        } else {
            break;
        }
    }

    max
}
