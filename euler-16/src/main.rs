use num_bigint::BigUint;

// TODO: use own bigint

fn main() {
    const POW: u32 = 1000;
    let mut num = BigUint::from(1u32) << POW;
    let mut sum = 0;
    while num != BigUint::ZERO {
        let rem: BigUint = &num % 10u32;
        sum += rem.iter_u32_digits().next().unwrap_or(0);
        num /= 10u32;
    }
    println!("{sum}");
}
