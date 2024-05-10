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

fn count_adj<T: PartialEq>(vec: Vec<T>) -> Vec<(usize, T)> {
    let mut result = Vec::new();
    for item in vec {
        match result.last() {
            Some((_, a)) if *a == item => {
                result.last_mut().unwrap().0 += 1;
            }
            _ => {
                result.push((1, item));
            }
        }
    }
    result
}

pub fn factor_count(num: u64) -> Vec<(usize, u64)> {
    count_adj(factor(num))
}

pub fn n_primes(n: usize) -> Vec<u64> {
    let mut primes = vec![2];
    let mut count = 3;
    while primes.len() < n {
        'block: {
            for p in &primes {
                if count % p == 0 {
                    count += 2;
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
                    count += 2;
                    break 'block;
                }
            }
            primes.push(count);
            count += 2;
        }
    }
    primes
}

pub fn divisors(n: u64) -> Vec<u64> {
    let factors = factor_count(n);
    let mut divisors = Vec::new();
    let max_powers: Vec<usize> = factors.iter().map(|x| x.0).collect();
    let mut powers = vec![0; max_powers.len()];
    while powers != max_powers {
        // calculate the divisor
        let mut divisor = 1;
        for (i, power) in powers.iter().enumerate() {
            for _ in 0..*power {
                divisor *= factors[i].1;
            }
        }
        divisors.push(divisor);

        for (i, num) in powers.iter_mut().enumerate() {
            if *num < max_powers[i] {
                *num += 1;
                for p in powers[..i].iter_mut() {
                    *p = 0;
                }
                break;
            }
        }
    }

    divisors
}
