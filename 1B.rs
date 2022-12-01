fn main(){
    
    let mut candies: Vec<u32> = Vec::new();
    
    for elf in include_str!("inputs/1.txt").split("\n\n") {
        let c = elf.split("\n").map(|x| x.parse::<u32>().unwrap()).sum();
        candies.push(c);
    }
    candies.sort();

    let mut max_candies:u32 = 0;
    for i in candies.len()-3..candies.len() {
        max_candies += candies[i];
    }

    println!("{max_candies}");
}

// 212489