fn run() -> u64 {
    const LIM: u64 = 100;
    let sum_squared = (LIM * (LIM + 1) / 2).pow(2);
    let sum_of_squares = LIM * (LIM + 1) * (2 * LIM + 1) / 6;
    sum_squared - sum_of_squares
}

fn main() {
    println!("{}", run());
}
