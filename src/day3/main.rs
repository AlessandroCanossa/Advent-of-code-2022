use std::fs;

fn main() {
    let lines = fs::read_to_string("src/day3/input.txt").unwrap();

    // task 1
    let tot: u32 = lines
        .lines()
        .map(|line| {
            let mid = line.len() / 2;

            let comps = line.split_at(mid);

            let c = comps.0.chars().find(|c| comps.1.contains(*c)).unwrap();

            match c {
                'a'..='z' => c as u32 - 'a' as u32 + 1,
                'A'..='Z' => c as u32 - 'A' as u32 + 27,
                _ => 0,
            }
        })
        .sum();

    println!("Total value of priority: {}", tot);

    // task2
    let lines: Vec<&str> = lines.lines().collect();
    let tot_priority: u32 = (0..lines.len())
        .step_by(3)
        .map(|i| {
            let line = &lines[i];

            let c = line
                .chars()
                .find(|c| lines[i + 1].contains(*c) && lines[i + 2].contains(*c))
                .unwrap();

            match c {
                'a'..='z' => c as u32 - 'a' as u32 + 1,
                'A'..='Z' => c as u32 - 'A' as u32 + 27,
                _ => 0,
            }
        })
        .sum();

    println!("Total priority of badges: {}", tot_priority);
}
