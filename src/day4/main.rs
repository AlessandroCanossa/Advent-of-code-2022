use std::fs;

fn main() {
    let input = fs::read_to_string("src/day4/input.txt").unwrap();

    let tot: u32 = input
        .lines()
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();

            let vals1: Vec<u32> = ranges[0]
                .split('-')
                .map(|val| val.parse::<u32>().unwrap())
                .collect();

            let vals2: Vec<u32> = ranges[1]
                .split('-')
                .map(|val| val.parse::<u32>().unwrap())
                .collect();

            // if vals1[0] >= vals2[0] && vals1[1] <= vals2[1]
            //     || vals2[0] >= vals1[0] && vals2[1] <= vals1[1]
            // {
            //     return 1;
            // }

            if (vals1[0]..=vals1[1]).contains(&vals2[0])
                || (vals1[0]..=vals1[1]).contains(&vals2[1])
                || (vals2[0]..=vals2[1]).contains(&vals1[0])
                || (vals2[0]..=vals2[1]).contains(&vals1[1])
            {
                return 1;
            }

            0
        })
        .sum();

    println!("Pairs: {}", tot);
}
