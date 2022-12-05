use crate::day5;

pub fn main() {
    let (mut columns, comands) = day5::readfiles::read();

    for comand in comands {
        let mut block = columns[(comand[1] - 1) as usize][0..=(comand[0] - 1) as usize].to_vec();

        for _ in 0..comand[0] {
            columns[(comand[1] - 1) as usize].remove(0);
        }

        block.append(&mut columns[(comand[2] - 1) as usize]);

        columns[(comand[2] - 1) as usize] = block;
    }

    print!("Task 1: ");
    for i in 0..columns.len() {
        print!("{}", columns[i][0]);
    }
    println!();
}
