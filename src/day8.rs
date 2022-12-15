use itertools::Itertools;

fn parse(inp: &str) -> Vec<Vec<u32>> {
    inp.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

pub fn solve1(inp: &str) -> usize {
    let grid = parse(inp);
    let len = grid.len();

    // Loop through coordinates of all trees that are not at an edge
    (1..len - 1)
    .cartesian_product(1..len - 1)
    .map(|(y, x)| {
        // Is this tree visible from any direction? Map to True/False
        directions(&grid, x, y)
                .iter()
                .map(|direction| direction.iter().all(|h| *h < grid[y][x]))
                .any(|visible| visible)
    })
    .filter(|visible| *visible)
    .count()
    // Add number of trees at edges, as they all are visible
    + (len - 1) * 4
}

pub fn solve2(inp: &str) -> usize {
    let trees = parse(inp);
    let len = trees.len();

    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            directions(&trees, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap()
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/8.txt")));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/8.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(
            solve1(
                "30373
25512
65332
33549
35390"
            ),
            21
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solve2(
                "30373
25512
65332
33549
35390"
            ),
            8
        );
    }
}
