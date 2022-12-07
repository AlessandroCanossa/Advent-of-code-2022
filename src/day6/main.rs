use std::fs;

// const MARKER_LEN: usize = 14; // packet
const MARKER_LEN: usize = 14; // message

fn main() {
    let data = fs::read_to_string("src/day6/input.txt").unwrap();

    let idx = (0..data.len() - MARKER_LEN)
        .find(|i| {
            let marker = data.get(*i..*i + MARKER_LEN).unwrap_or_default();
            let mut marker: Vec<char> = marker.chars().collect();
            marker.sort();
            marker.dedup();

            marker.len() == MARKER_LEN
        })
        .unwrap_or(0);

    println!("{}", idx + MARKER_LEN);
}
