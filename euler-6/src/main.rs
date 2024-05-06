fn main() {
    const LIM: u64 = 100;
    let sum_squared = (LIM * (LIM + 1) / 2).pow(2);
    let sum_of_squares = LIM * (LIM + 1) * (2 * LIM + 1) / 6;
    println!("{}", sum_squared - sum_of_squares);
}
