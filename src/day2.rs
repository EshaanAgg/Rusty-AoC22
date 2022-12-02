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
        || (res[0] == "C" && res[1] == "Z")
    {
        score += 6
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
