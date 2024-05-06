use euler_util::primes_under;

fn main() {
    println!("{}", primes_under(2_000_000).into_iter().sum::<u64>());
}
