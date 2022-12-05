use std::fs;

fn main() {
    let input_lines = fs::read_to_string("src/day2/input.txt").unwrap();

    let tot_score: u32 = input_lines.lines().map(|line| {
        let shapes: Vec<&str> = line.split(' ').collect();

        match shapes[1] {
            "X" => match shapes[0] { // lose
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0,
            },

            "Y" => match shapes[0] { // draw
                "A" => 4,
                "B" => 5,
                "C" => 6,
                _ => 0,
            },

            "Z" => match shapes[0] { // win
                "A" => 8,
                "B" => 9,
                "C" => 7,
                _ => 0,
            },
            _ => 0,
        }
    }).sum();


    println!("Total points: {}", tot_score);
}
