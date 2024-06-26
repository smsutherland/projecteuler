use euler_util::divisors;

const NUM: u64 = 10000;

fn run() -> u64 {
    let mut sum = 0;
    for a in 2..NUM {
        let b = divisors(a).iter().sum();
        if b != a {
            let c: u64 = divisors(b).iter().sum();
            if c == a {
                sum += a;
            }
        }
    }
    sum
}

fn main() {
    println!("{}", run());
}
