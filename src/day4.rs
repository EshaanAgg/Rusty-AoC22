use std::cmp::max;
use std::cmp::min;

pub fn part1() {
    println!(
        "{}",
        include_str!("inputs/4.txt")
            .lines()
            .map(|inp| inp
                .split(",")
                .map(|x| x
                    .split("-")
                    .map(|u| u.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>())
                .collect::<Vec<_>>())
            .map(|line| {
                if (line[0][0] >= line[1][0] && line[0][1] <= line[1][1])
                    || (line[0][0] <= line[1][0] && line[0][1] >= line[1][1])
                {
                    return 1;
                } else {
                    return 0;
                }
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    )
}

pub fn part2() {
    println!(
        "{}",
        include_str!("inputs/4.txt")
            .lines()
            .map(|inp| inp
                .split(",")
                .map(|x| x
                    .split("-")
                    .map(|u| u.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>())
                .collect::<Vec<_>>())
            .map(|line| {
                if max(line[0][0], line[1][0]) <= min(line[0][1], line[1][1]) {
                    return 1;
                } else {
                    return 0;
                }
            })
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    )
}
