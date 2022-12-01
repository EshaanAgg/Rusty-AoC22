use std::time;
mod day1;

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("Time used: {:?}", time::Instant::now().duration_since(t0));
    println!()
    
    ret
}

fn main() {
    bench(day1::part1);
    bench(day1::part2);
}