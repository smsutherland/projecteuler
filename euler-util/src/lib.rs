pub fn factor(mut num: u64) -> Vec<u64> {
    let twos = num.trailing_zeros();
    let mut factors = vec![2; twos as usize];
    num >>= twos;
    let mut i = 3;
    while i <= num {
        if num % i == 0 {
            factors.push(i);
            num /= i;
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
    if n == 0 {
        return Vec::new();
    }
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

/// From the rust standard library. It's not stabilized yet.
macro_rules! isqrt {
    ($name:ident: $t:ty) => {
        pub const fn $name(n: $t) -> $t {
            if n < 2 {
                return n;
            }

            // The algorithm is based on the one presented in
            // <https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)>
            // which cites as source the following C code:
            // <https://web.archive.org/web/20120306040058/http://medialab.freaknet.org/martin/src/sqrt/sqrt.c>.

            let mut op = n;
            let mut res = 0;
            let mut one = 1 << (n.ilog2() & !1);

            while one != 0 {
                if op >= res + one {
                    op -= res + one;
                    res = (res >> 1) + one;
                } else {
                    res >>= 1;
                }
                one >>= 2;
            }

            // SAFETY: the result is positive and fits in an integer with half as many bits.
            // Inform the optimizer about it.
            // unsafe {
            //     std::hint::assert_unchecked(0 < res);
            //     std::hint::assert_unchecked(res < 1 << ($t::BITS / 2));
            // }

            res
        }
    };
}

isqrt!(u64isqrt: u64);

/// Based on Sieve of Atkin
pub fn primes_under(lim: u64) -> Vec<u64> {
    use bitvec::prelude::*;

    let lim = lim + 1;

    let root = u64isqrt(lim);
    let s = [1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];

    let mut result = Vec::new();
    let mut is_prime: BitVec = bitvec![0; lim as usize+1];

    if lim > 2 {
        result.push(2);
    }
    if lim > 3 {
        result.push(3);
    }
    if lim > 5 {
        result.push(5);
    }

    let remainder_groups: [&[_]; 3] = [
        &[1, 13, 17, 29, 37, 41, 49, 53],
        &[7, 19, 31, 43],
        &[11, 23, 47, 59],
    ];
    let mut remainder_buckets = [bitvec![0; 60], bitvec![0; 60], bitvec![0; 60]];
    for (bucket, remainder) in remainder_buckets
        .iter_mut()
        .zip(remainder_groups.iter().copied())
    {
        for r in remainder {
            bucket.set(*r as usize, true);
        }
    }

    for x in 1..=root {
        let xx4 = 4 * x * x;
        for y in (1..=root).step_by(2) {
            let n = xx4 + y * y;
            if n > lim {
                break;
            }
            if remainder_buckets[0][n as usize % 60] {
                let new = !is_prime[n as usize];
                is_prime.set(n as usize, new);
            }
        }
    }

    for x in (1..=root).step_by(2) {
        let xx3 = 3 * x * x;
        for y in (2..=root).step_by(2) {
            let n = xx3 + y * y;
            if n > lim {
                break;
            }
            if remainder_buckets[1][n as usize % 60] {
                let new = !is_prime[n as usize];
                is_prime.set(n as usize, new);
            }
        }
    }

    for x in 2..=root {
        let xx3 = 3 * x * x;
        for y in (x % 2 + 1..x).step_by(2) {
            let n = xx3.saturating_sub(y * y);
            if n == 0 {
                break;
            }
            if n > lim {
                continue;
            }
            if remainder_buckets[2][n as usize % 60] {
                let new = !is_prime[n as usize];
                is_prime.set(n as usize, new);
            }
        }
    }

    for i in (0..root).step_by(60) {
        for x in &s {
            let n = i + x;
            if n as usize >= is_prime.len() {
                continue;
            }
            if is_prime[n as usize] {
                let nn = n * n;
                for j in (0..lim / nn).step_by(60) {
                    for y in &s {
                        let c = j + y;
                        if nn * c > lim {
                            break;
                        }
                        is_prime.set((nn * c) as usize, false);
                    }
                }
            }
        }
    }

    for i in (0..lim).step_by(60) {
        for x in &s {
            let n = i + x;
            if n <= lim && is_prime[n as usize] {
                result.push(n);
            }
        }
    }

    result
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
