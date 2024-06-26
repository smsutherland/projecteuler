use euler_util::factor;

fn run() -> u64 {
    const NUMBER: u64 = 600851475143;
    let factors = factor(NUMBER);
    factors[factors.len() - 1]
}

fn main() {
    println!("{}", run());
}
