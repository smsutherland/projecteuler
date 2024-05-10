use num_bigint::BigUint;

const LENGTH: usize = 1000;

fn run() -> usize {
    let mut a = BigUint::from(1u32);
    let mut b = BigUint::from(1u32);
    let mut c;
    let mut i = 2;
    while b.to_string().len() < LENGTH {
        c = &a + &b;
        a = b;
        b = c;
        i += 1;
    }
    i
}

fn main() {
    println!("{}", run());
}

#[test]
fn name() {
    assert_eq!(run(), 4782);
}
