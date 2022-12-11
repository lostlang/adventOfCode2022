use crate::day11;

pub fn main() {
    let mut monkeys = day11::readfiles::read();
    let mut moves = vec![0; monkeys.len()];

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            let mut item_move: Vec<(i32, i32)> = Vec::new();

            for item in &monkeys[monkey].items {
                let move_to = monkeys[monkey].move_to;
                let operand = (
                    monkeys[monkey].operands.0.get_value(*item),
                    monkeys[monkey].operands.1.get_value(*item),
                );
                let new_item = monkeys[monkey].operation.get_value(operand) / 3;

                moves[monkey] += 1;

                if new_item % monkeys[monkey].test == 0 {
                    item_move.push((move_to.0, new_item));
                } else {
                    item_move.push((move_to.1, new_item));
                }
            }

            for (i, item) in item_move {
                monkeys[i as usize].items.push(item);
            }

            monkeys[monkey].items = Vec::new();
        }
    }

    moves.sort();
    moves.reverse();

    println!("Task 1 : {:?}", moves[0] * moves[1]);
}
