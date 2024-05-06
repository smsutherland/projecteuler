fn main() {
    const LIM: usize = 10001;
    let mut primes = vec![2];
    let mut count = 3;
    while primes.len() < LIM {
        'block: {
            for p in &primes {
                if count % p == 0 {
                    count += 1;
                    break 'block;
                }
            }
            primes.push(count);
            count += 2;
        }
    }
    println!("{}", primes.last().unwrap());
}
