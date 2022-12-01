use std::cmp::max;

fn main(){
    
    let mut max_candies:u32 = 0;
    
    for elf in include_str!("inputs/1.txt").split("\n\n") {
        let candies = elf.split("\n").map(|x| x.parse::<u32>().unwrap()).sum();
        max_candies = max(max_candies, candies);
    }

    println!("{max_candies}");
}

// 71780