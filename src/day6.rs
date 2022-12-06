use std::collections::HashSet;

fn solve1(inp: &str) -> usize {
    inp.chars()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(4)
        .filter(|w| w.iter().map(|p| p.1).collect::<HashSet<_>>().len() == 4)
        .next()
        .unwrap()[3]
        .0
        + 1
}

fn solve2(inp: &str) -> usize {
    inp.chars()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(14)
        .filter(|w| w.iter().map(|p| p.1).collect::<HashSet<_>>().len() == 14)
        .next()
        .unwrap()[13]
        .0
        + 1
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/6.txt")));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/6.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(solve1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn sample2() {
        assert_eq!(solve2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
