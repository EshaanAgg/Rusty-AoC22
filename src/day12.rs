extern crate queues;
use queues::*;

fn parse(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|line| line.chars().collect()).collect()
}

fn solve1(inp: &str) -> i32 {
    let dx: Vec<i32> = vec![0, 0, 1, -1];
    let dy: Vec<i32> = vec![1, -1, 0, 0];

    let mut grid = parse(inp).clone();
    let y_max = grid.len() as i32;
    let x_max = grid[0].len() as i32;

    let mut distance: Vec<Vec<i32>> = vec![vec![-1; x_max as usize]; y_max as usize];

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for y in 0..y_max {
        for x in 0..x_max {
            if grid[y as usize][x as usize] == 'S' {
                start = (x as usize, y as usize);
                grid[y as usize][x as usize] = 'a';
            }
            if grid[y as usize][x as usize] == 'E' {
                end = (x as usize, y as usize);
                grid[y as usize][x as usize] = 'z';
            }
        }
    }

    let mut q: Queue<(usize, usize)> = queue![];
    distance[start.1][start.0] = 0;
    q.add(start);

    while q.size() > 0 {
        let top = q.peek().unwrap();
        q.remove();
        if top == end {
            break;
        }
        for i in 0..4 {
            let xn = (top.0 as i32) + dx[i];
            let yn = (top.1 as i32) + dy[i];
            if xn >= 0
                && yn >= 0
                && yn < y_max
                && xn < x_max
                && distance[yn as usize][xn as usize] == -1
                && ((grid[yn as usize][xn as usize] as i32) - (grid[top.1][top.0] as i32) <= 1)
            {
                distance[yn as usize][xn as usize] = distance[top.1][top.0] + 1;
                q.add((xn as usize, yn as usize));
            }
        }
    }

    distance[end.1][end.0]
}

fn solve2(inp: &str) -> i32 {
    let dx: Vec<i32> = vec![0, 0, 1, -1];
    let dy: Vec<i32> = vec![1, -1, 0, 0];

    let mut grid = parse(inp).clone();
    let y_max = grid.len() as i32;
    let x_max = grid[0].len() as i32;

    let mut distance: Vec<Vec<i32>> = vec![vec![-1; x_max as usize]; y_max as usize];

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut q: Queue<(usize, usize)> = queue![];

    for y in 0..y_max {
        for x in 0..x_max {
            if grid[y as usize][x as usize] == 'S' {
                start = (x as usize, y as usize);
                grid[y as usize][x as usize] = 'a';
            }
            if grid[y as usize][x as usize] == 'E' {
                end = (x as usize, y as usize);
                grid[y as usize][x as usize] = 'z';
            }

            if grid[y as usize][x as usize] == 'a' {
                distance[y as usize][x as usize] = 0;
                q.add((x as usize, y as usize));
            }
        }
    }

    while q.size() > 0 {
        let top = q.peek().unwrap();
        q.remove();
        if top == end {
            break;
        }
        for i in 0..4 {
            let xn = (top.0 as i32) + dx[i];
            let yn = (top.1 as i32) + dy[i];
            if xn >= 0
                && yn >= 0
                && yn < y_max
                && xn < x_max
                && distance[yn as usize][xn as usize] == -1
                && ((grid[yn as usize][xn as usize] as i32) - (grid[top.1][top.0] as i32) <= 1)
            {
                distance[yn as usize][xn as usize] = distance[top.1][top.0] + 1;
                q.add((xn as usize, yn as usize));
            }
        }
    }

    distance[end.1][end.0]
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/12.txt")));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/12.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn sample1() {
        assert_eq!(solve1(CASE1), 31);
    }

    #[test]
    fn sample2() {
        assert_eq!(solve2(CASE1), 29);
    }
}
