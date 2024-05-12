const COINS: [u8; 8] = [200, 100, 50, 20, 10, 5, 2, 1];
const GOAL: u8 = 200;

fn count(coins: &[u8], target: u8) -> usize {
    if coins.len() == 1 {
        return 1;
    }

    let mut i = 0;
    let mut res = 0;
    while coins[0].saturating_mul(i) <= target {
        res += count(&coins[1..], target - coins[0] * i);
        i += 1;
    }
    res
}

fn run() -> usize {
    count(&COINS[..], GOAL)
}

fn main() {
    println!("{}", run());
}
