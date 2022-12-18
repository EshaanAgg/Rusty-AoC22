#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct MachineState {
    cycle: i32,
    x: i32,
}

fn parse(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .map(|line| {
            if let Some(("addx", num)) = line.split_once(' ') {
                Instruction::Addx(num.parse().unwrap())
            } else {
                Instruction::Noop
            }
        })
        .collect()
}

fn solve1(inp: &str) -> i32 {
    let instructions = parse(inp);

    let mut state = MachineState { cycle: 0, x: 1 };
    let mut strength = 0;

    instructions.into_iter().for_each(|instruction| {
        state.cycle += 1;
        if state.cycle <= 220 && state.cycle % 40 == 20 {
            strength += state.cycle * state.x;
        };

        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(x) => {
                state.cycle += 1;
                if state.cycle <= 220 && state.cycle % 40 == 20 {
                    strength += state.cycle * state.x;
                };
                state.x += x;
            }
        }
    });

    strength
}

const COLS: i32 = 40;
const ROWS: i32 = 6;
const SPRITE_WIDTH: u32 = 3;

fn get_pixel(state: &MachineState) -> char {
    let curr_col = (state.cycle - 1) % COLS;
    if curr_col.abs_diff(state.x) <= SPRITE_WIDTH / 2 {
        '█'
    } else {
        ' '
    }
}

fn solve2(inp: &str) -> String {
    let instructions = parse(inp);
    let mut screen = [' '; (COLS * ROWS) as usize];

    let mut state = MachineState { cycle: 0, x: 1 };

    instructions.into_iter().for_each(|instruction| {
        screen[state.cycle as usize] = get_pixel(&state);
        state.cycle += 1;

        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(x) => {
                screen[state.cycle as usize] = get_pixel(&state);
                state.cycle += 1;
                state.x += x;
            }
        }
    });

    let image = screen
        .chunks(COLS as usize)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");

    image
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/10.txt")));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/10.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASE1: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn sample1() {
        assert_eq!(solve1(CASE1), 13140);
    }
}
