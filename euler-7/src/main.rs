use euler_util::n_primes;

fn main() {
    let primes = n_primes(10001);
    println!("{}", primes.last().unwrap());
}
