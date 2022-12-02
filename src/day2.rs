use core::panic;

fn outcome(res: Vec<&str>) -> u32 {
    let mut score: u32 = match res[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    };

    if (res[0] == "A" && res[1] == "X")
        || (res[0] == "B" && res[1] == "Y")
        || (res[0] == "C" && res[1] == "Z")
    {
        score += 3;
    } else if (res[0] == "A" && res[1] == "Y")
        || (res[0] == "B" && res[1] == "Z")
        || (res[0] == "C" && res[1] == "X")
    {
        score += 6
    }

    score
}

fn outcome2(res: Vec<&str>) -> u32 {
    let mut score: u32 = match res[1] {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    };

    if res[1] == "X" {
        score += match res[0] {
            "A" => 3,
            "B" => 1,
            "C" => 2,
            _ => panic!(),
        }
    } else if res[1] == "Y" {
        score += match res[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!(),
        }
    } else {
        score += match res[0] {
            "A" => 2,
            "B" => 3,
            "C" => 1,
            _ => panic!(),
        }
    }

    score
}

pub fn part1() {
    let mut score: u32 = 0;

    for mat in include_str!("inputs/2.txt").split("\n") {
        let res: Vec<&str> = mat.split(" ").collect();
        score += outcome(res);
    }

    println!("{score}");
}

pub fn part2() {
    let mut score: u32 = 0;

    for mat in include_str!("inputs/2.txt").split("\n") {
        let res: Vec<&str> = mat.split(" ").collect();
        score += outcome2(res);
    }

    println!("{score}");
}
