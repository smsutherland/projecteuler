use euler_util::is_prime;

fn count_primes(a: i64, b: i64) -> u64 {
    let mut n = 0;
    let f = move |n| n * n + a * n + b;
    while {
        let fofn = f(n);
        fofn >= 0 && is_prime(f(n) as u64)
    } {
        n += 1;
    }
    n as u64
}

fn run() -> i64 {
    let mut max_primes = 0;
    let mut max = 0;
    for a in -999..1000 {
        for b in 2..=1000 {
            let primes = count_primes(a, b);
            if primes > max_primes {
                max_primes = primes;
                max = a * b;
            }
        }
    }
    max
}

fn main() {
    println!("{}", run());
}
