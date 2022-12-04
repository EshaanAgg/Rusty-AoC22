use std::time;
mod day4;

fn bench(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("Time used: {:?}", time::Instant::now().duration_since(t0));
    println!();

    ret
}

fn main() {
    bench(day4::part1);
    bench(day4::part2);
}
