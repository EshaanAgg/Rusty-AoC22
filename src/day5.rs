use itertools::Itertools;
use scan_fmt::scan_fmt;

struct Move {
    n: usize,
    from: usize,
    to: usize,
}

type ParsedInput = (Vec<Vec<char>>, Vec<Move>);

fn parse_input(input: &str) -> ParsedInput {
    let mut split = input.split("\n\n");

    let parse = split.next().unwrap().lines().collect::<Vec<&str>>();
    let (crate_row, towers) = parse.split_last().unwrap();
    let moves = split.next().unwrap();

    let size: usize = (crate_row.len() + 1) / 4;
    let mut rows: Vec<Vec<char>> = vec![Vec::new(); size];

    towers.into_iter().for_each(|row| {
        row.chars()
            .enumerate()
            .filter(|(_, c)| !(c.is_whitespace() || *c == '[' || *c == ']'))
            .for_each(|(i, c)| rows[(i - 1) / 4].push(c))
    });

    (
        rows.into_iter()
            .map(|mut arr| {
                arr.reverse();
                arr
            })
            .collect::<Vec<Vec<char>>>(),
        moves
            .lines()
            .map(|l| {
                let (n, from, to) =
                    scan_fmt!(l, "move {} from {} to {}", usize, usize, usize).unwrap();
                Move { n, from, to }
            })
            .collect(),
    )
}

fn solve1(inp: ParsedInput) -> String {
    let (towers, moves) = inp;
    let mut towers = towers.clone();

    moves.iter().for_each(|m| {
        let l = towers[m.from - 1].len();
        towers[m.from - 1]
            .drain(l - m.n..)
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .for_each(|c| towers[m.to - 1].push(*c));
    });

    towers.iter().map(|t| t[t.len() - 1]).join("")
}

fn solve2(inp: ParsedInput) -> String {
    let (towers, moves) = inp;
    let mut towers = towers.clone();

    moves.iter().for_each(|m| {
        let l = towers[m.from - 1].len();
        towers[m.from - 1]
            .drain(l - m.n..)
            .collect::<Vec<char>>()
            .iter()
            .for_each(|c| towers[m.to - 1].push(*c));
    });

    towers.iter().map(|t| t[t.len() - 1]).join("")
}

pub fn part1() {
    println!("{}", solve1(parse_input(include_str!("inputs/5.txt"))))
}

pub fn part2() {
    println!("{}", solve2(parse_input(include_str!("inputs/5.txt"))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn sample1() {
        assert_eq!(solve1(parse_input(input())), "CMZ");
    }

    #[test]
    fn sample2() {
        assert_eq!(solve2(parse_input(input())), "MCD");
    }
}
