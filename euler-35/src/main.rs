use euler_util::primes_under;
use std::collections::HashSet;

const LIM: u64 = 1000000;

fn log10(mut n: u64) -> u32 {
    let mut len = 0;
    while n > 0 {
        n /= 10;
        len += 1
    }
    len
}

fn rotate(mut n: u64) -> u64 {
    let len = log10(n);
    let last_digit = n % 10;
    n /= 10;
    n += (10u64).pow(len - 1) * last_digit;
    n
}

#[test]
fn test_rotate() {
    assert_eq!(rotate(1), 1);
    assert_eq!(rotate(12), 21);
    assert_eq!(rotate(583), 358);
}

fn run() -> usize {
    let mut primes: HashSet<_> = primes_under(LIM).into_iter().collect();

    for _ in 0..log10(LIM) - 1 {
        primes = primes
            .iter()
            .copied()
            .map(rotate)
            .filter(|n| primes.contains(n))
            .collect();
    }

    primes.len()
}

fn main() {
    println!("{}", run());
}
