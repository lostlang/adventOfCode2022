use crate::day1;

pub fn main() {
    let values = day1::readfiles::read();

    let mut max = (0, 0, 0);
    let mut current = 0;

    for v in values {
        match v {
            Some(x) => {
                current += x;
            }
            None => {
                if current > max.0 {
                    max = (current, max.1, max.2);
                };

                max = sort(max);

                current = 0;
            }
        }
    }

    println!("Task 2: {}", max.0 + max.1 + max.2);
}

fn sort(list: (u32, u32, u32)) -> (u32, u32, u32) {
    let mut list = list;

    if list.0 > list.1 {
        let tmp = list.0;
        list.0 = list.1;
        list.1 = tmp;
    }

    if list.1 > list.2 {
        let tmp = list.1;
        list.1 = list.2;
        list.2 = tmp;
    }

    if list.0 > list.1 {
        let tmp = list.0;
        list.0 = list.1;
        list.1 = tmp;
    }

    list
}
