fn run() -> u64 {
    for a in 1..1000 {
        for b in 1..(1000 - a) {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    panic!("no valid triplet found");
}

fn main() {
    println!("{}", run());
}
