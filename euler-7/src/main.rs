use euler_util::n_primes;

fn run() -> u64 {
    let primes = n_primes(10001);
    primes[primes.len() - 1]
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_7() {
    assert_eq!(run(), 104743);
}
