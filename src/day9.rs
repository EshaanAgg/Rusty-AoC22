use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GridPos {
    x: i32,
    y: i32,
}

impl fmt::Debug for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for GridPos {
    fn add_assign(&mut self, other: GridPos) {
        *self = GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl GridPos {
    fn is_touching(self, other: GridPos) -> bool {
        let diff = self - other;
        !(diff.x.abs() > 1 || diff.y.abs() > 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> GridPos {
        match self {
            Direction::Up => GridPos { x: 0, y: 1 },
            Direction::Down => GridPos { x: 0, y: -1 },
            Direction::Left => GridPos { x: -1, y: 0 },
            Direction::Right => GridPos { x: 1, y: 0 },
        }
    }

    fn get_dir(dir: char) -> Direction {
        match dir {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Direction not recognised"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    dist: u32,
}

fn parse(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .map(|line| {
            let (dir, dist) = scan_fmt!(line, "{} {}", char, u32).unwrap();
            Instruction {
                dir: Direction::get_dir(dir),
                dist,
            }
        })
        .collect()
}

fn solve1(inp: &str) -> usize {
    let instructions = parse(inp);

    let start = GridPos { x: 0, y: 0 };

    let mut head = start;
    let mut tail = start;

    let mut visited = HashSet::new();
    visited.insert(tail);

    instructions.into_iter().for_each(|instruction| {
        for _ in 0..instruction.dist {
            head += instruction.dir.delta();

            if !head.is_touching(tail) {
                tail.x += (head - tail).x.signum();
                tail.y += (head - tail).y.signum();
                visited.insert(tail);
            }
        }
    });

    visited.len()
}

fn solve2(inp: &str) -> usize {
    let instructions = parse(inp);

    let start = GridPos { x: 0, y: 0 };

    // rope[0] is the Head and rope[9] is the Tail
    let mut rope: Vec<GridPos> = vec![start; 10];

    let mut visited: HashSet<GridPos> = HashSet::new();
    visited.insert(start);

    instructions.into_iter().for_each(|instruction| {
        for _ in 0..instruction.dist {
            rope[0] += instruction.dir.delta();

            for i in 0..9 {
                if !rope[i + 1].is_touching(rope[i]) {
                    rope[i + 1].x += (rope[i] - rope[i + 1]).x.signum();
                    rope[i + 1].y += (rope[i] - rope[i + 1]).y.signum();
                }
            }

            visited.insert(rope[9]);
        }
    });

    visited.len()
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/9.txt")));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/9.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            solve1(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            13
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solve2(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            1
        );
        assert_eq!(
            solve2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            36
        );
    }
}
