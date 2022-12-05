#![warn(clippy::all, clippy::pedantic)]

use std::fs;

fn get_max(elves: &[u32]) -> (usize, u32) {
    elves
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(id, val)| (id, *val))
        .unwrap()
}

fn main() {
    let lines = fs::read_to_string("src/day1/data.txt").unwrap();

    let mut elves: Vec<u32> = vec![0; 1];

    lines.lines().for_each(|line| {
        if line.is_empty() {
            elves.push(0);
            return;
        }

        let id = elves.len() - 1;
        elves[id] += line.parse::<u32>().unwrap();
    });

    let (id, first_max) = get_max(&elves);
    elves.remove(id);
    let (id, second_max) = get_max(&elves);
    elves.remove(id);
    let (_, third_max) = get_max(&elves);

    let max_cal = first_max + second_max + third_max;

    println!("Total cal of first three elves {}.", max_cal);
}
