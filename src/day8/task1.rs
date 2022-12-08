use crate::day8;

pub fn main() {
    let mut values = day8::readfiles::read();
    let reference = day8::readfiles::read();

    let mut count = 0;

    for i in 0..values.len() {
        let mut max = 0;

        for j in 0..values[i].len() {
            if reference[i][j] > max {
                max = reference[i][j];
                values[i][j] = 0;
            }
        }
    }

    for i in 0..values.len() {
        let mut max = 0;

        for j in 0..values[i].len() {
            if reference[j][i] > max {
                max = reference[j][i];
                values[j][i] = 0;
            }
        }
    }

    for i in (0..values.len()).rev() {
        let mut max = 0;

        for j in (0..values[i].len()).rev() {
            if reference[i][j] > max {
                max = reference[i][j];
                values[i][j] = 0;
            }
        }
    }

    for i in (0..values.len()).rev() {
        let mut max = 0;

        for j in (0..values[i].len()).rev() {
            if reference[j][i] > max {
                max = reference[j][i];
                values[j][i] = 0;
            }
        }
    }

    for i in 0..values.len() {
        for j in 0..values[i].len() {
            if values[i][j] == 0 {
                count += 1;
            }
        }
    }

    println!("Task 1: {}", count);
}
