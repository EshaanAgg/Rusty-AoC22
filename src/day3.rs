use std::collections::HashSet;

fn get_score(letter: char) -> u32 {
    if letter.is_lowercase() {
        return letter as u32 - 97 + 1;
    } else {
        return letter as u32 - 65 + 27;
    }
}

pub fn part1() {
    let mut priority: u32 = 0;

    for sack in include_str!("inputs/3.txt").split("\n") {
        let n = sack.chars().count();
        let mut dic: HashSet<char> = HashSet::new();
        let mut letters: HashSet<char> = HashSet::new();

        for (i, c) in sack.chars().enumerate() {
            if i < (n / 2) {
                dic.insert(c);
            } else if dic.contains(&c) {
                letters.insert(c);
            }
        }

        for c in letters {
            priority += get_score(c);
        }
    }

    println!("{priority}");
}

pub fn part2() {
    let priority: u32 = include_str!("inputs/3.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let first: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let second: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let third: HashSet<char> = HashSet::from_iter(chunk[2].chars());

            let mut intersection = first;
            intersection.retain(|c| second.contains(c));
            intersection.retain(|c| third.contains(c));

            let c = intersection.iter().next().unwrap();

            get_score(*c)
        })
        .sum();
    println!("{priority}");
}
