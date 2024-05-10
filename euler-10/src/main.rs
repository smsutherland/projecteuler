use euler_util::primes_under;

fn run() -> u64 {
    primes_under(2_000_000).into_iter().sum()
}

fn main() {
    println!("{}", run());
}

#[test]
fn euler_10() {
    assert_eq!(run(), 142913828922);
}
