const LIM: u64 = 1001;

fn run() -> u64 {
    const L2: u64 = LIM * LIM;
    const L3: u64 = L2 * LIM;

    (4 * L3 + 3 * L2 + 8 * LIM - 9) / 6
}

fn main() {
    println!("{}", run());
}
