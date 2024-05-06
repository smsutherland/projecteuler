pub fn factor(num: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = num;
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else if i == 2 {
            i += 1;
        } else {
            i += 2;
        }
    }
    factors
}

pub fn n_primes(n: usize) -> Vec<u64> {
    let mut primes = vec![2];
    let mut count = 3;
    while primes.len() < n {
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
    primes
}

pub fn primes_under(lim: u64) -> Vec<u64> {
    let mut primes = vec![2];
    let mut count = 3;
    while count < lim {
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
    primes
}
