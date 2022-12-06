use std::fs;

fn main() {
    let input = fs::read_to_string("src/day5/input.txt").unwrap();

    let input: Vec<&str> = input.lines().collect();

    let split_point = input
        .iter()
        .enumerate()
        .find_map(|(i, line)| if line.is_empty() { Some(i) } else { None })
        .unwrap();

    let mut stacks: Vec<Vec<char>> = (0..split_point)
        .map(|i| input[i].chars().collect())
        .collect();

    // task 1
    // input.iter().skip(split_point + 1).for_each(|line| {
    //     let parts: Vec<&str> = line.split(' ').collect();

    //     let quantity: usize = parts[1].parse().unwrap();
    //     let src: usize = parts[3].parse().unwrap() - 1;
    //     let dst: usize = parts[5].parse().unwrap() - 1;

    //     for _ in 0..quantity {
    //         let c = stacks[src].pop().unwrap();
    //         stacks[dst].push(c);
    //     }
    // });

    // task 2
    input.iter().skip(split_point + 1).for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();

        let quantity: usize = parts[1].parse().unwrap();
        let src: usize = parts[3].parse().unwrap();
        let src = src - 1;
        let dst: usize = parts[5].parse().unwrap();
        let dst = dst - 1;

        let split_id = stacks[src].len() - quantity;
        let mut app: Vec<char> = stacks[src].drain(split_id..).collect();
        stacks[dst].append(&mut app);
    });

    let stack_top: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    println!("{}", stack_top);
}
