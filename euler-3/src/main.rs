fn factor(num: u64) -> Vec<u64> {
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

fn main() {
    let number: u64 = 600851475143;
    println!("{}", factor(number).last().unwrap());
}
