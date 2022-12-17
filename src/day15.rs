use scan_fmt::scan_fmt;
use std::collections::HashSet;

fn parse(inp: &str) -> Vec<Vec<i32>> {
    inp.lines()
        .map(|l| {
            let (x1, y1, x2, y2) = scan_fmt!(
                l,
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            vec![x1, y1, x2, y2]
        })
        .collect::<Vec<Vec<i32>>>()
}

fn merge(mut ranges: Vec<(i32, i32)>) -> u32 {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let n = ranges.len();

    let mut s = Vec::<(i32, i32)>::new();
    s.push(ranges[0]);

    for i in 1..n {
        let top = *s.last().unwrap();

        if top.1 < ranges[i].0 {
            s.push(ranges[i]);
        } else if top.1 < ranges[i].1 {
            s.remove(s.len() - 1);
            s.push((top.0, ranges[i].1));
        }
    }

    let mut ans = 0;
    while s.len() != 0 {
        let t = s.last().unwrap();
        ans += t.1 - t.0 + 1;
        s.pop();
    }

    ans as u32
}

fn solve1(inp: &str, y: i32) -> u32 {
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    let data = parse(inp);
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    data.into_iter().for_each(|d| {
        let radius: i32 = (d[0].abs_diff(d[2]) + d[1].abs_diff(d[3])) as i32;
        let distance: i32 = radius - (y.abs_diff(d[1]) as i32);

        if distance >= 0 {
            ranges.push((d[0] - distance, d[0] + distance));
        }

        if d[3] == y {
            beacons.insert((d[2], d[3]));
        }
    });

    merge(ranges) - (beacons.len() as u32)
}

fn merge_ranges(mut ranges: Vec<(i32, i32)>) -> Vec<Vec<i32>> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let n = ranges.len();

    let mut s = Vec::<(i32, i32)>::new();
    s.push(ranges[0]);

    for i in 1..n {
        let top = *s.last().unwrap();

        if top.1 < ranges[i].0 {
            s.push(ranges[i]);
        } else if top.1 < ranges[i].1 {
            s.remove(s.len() - 1);
            s.push((top.0, ranges[i].1));
        }
    }

    let mut ans: Vec<Vec<i32>> = Vec::new();
    while s.len() != 0 {
        let t = s.last().unwrap();
        ans.push(Vec::from([t.0, t.1]));
        s.pop();
    }

    ans
}

fn row_ranges(row: i32, data: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ranges: Vec<(i32, i32)> = Vec::new();

    data.into_iter().for_each(|d| {
        let radius: i32 = (d[0].abs_diff(d[2]) + d[1].abs_diff(d[3])) as i32;
        let distance: i32 = radius - (row.abs_diff(d[1]) as i32);

        if distance >= 0 {
            ranges.push((d[0] - distance, d[0] + distance));
        }
    });

    merge_ranges(ranges)
}

fn solve2(inp: &str, size: i32) -> i64 {
    let data = parse(inp);

    let col_ranges = (0..=size)
        .map(|row| (row, row_ranges(row, &data)))
        .filter(|(_, ranges)| ranges.len() > 1)
        .collect::<Vec<(i32, Vec<Vec<i32>>)>>();

    let row = col_ranges.first().unwrap().0;
    let col = col_ranges.first().unwrap().1.first().unwrap()[1] + 1;
    println!("{} {}", row, col);
    i64::from(col) * 4_000_000 + i64::from(row)
}

pub fn part1() {
    println!("{}", solve1(include_str!("inputs/15.txt"), 2000000));
}

pub fn part2() {
    println!("{}", solve2(include_str!("inputs/15.txt"), 4000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample1() {
        assert_eq!(
            solve1(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                10
            ),
            26
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solve2(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                20
            ),
            56000011
        );
    }
}
