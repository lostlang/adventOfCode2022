use crate::day4;

pub fn main() {
    let values = day4::readfiles::read();

    let mut count = 0;

    for v in values {
        let mut vec = vec![v.0 .0, v.0 .1, v.1 .0, v.1 .1];
        vec.sort();

        if vec[0] == v.0 .0 && vec[1] == v.1 .0 || vec[0] == v.1 .0 && vec[1] == v.0 .0 {
            count += 1;
        }
    }

    println!("Task 2: {}", count);
}
