use num_bigint::BigUint;

const NUM: u32 = 100;

fn main() {
    let mut factorial = BigUint::from(1u32);
    for i in 1..=NUM {
        factorial *= i;
    }
    let mut sum = 0;
    while factorial != BigUint::ZERO {
        let rem: BigUint = &factorial % 10u32;
        sum += rem.iter_u32_digits().next().unwrap_or(0);
        factorial /= 10u32;
    }
    println!("{sum}");
}
