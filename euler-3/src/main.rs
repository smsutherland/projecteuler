use euler_util::factor;

fn main() {
    let number: u64 = 600851475143;
    println!("{}", factor(number).last().unwrap());
}
