const MAX: u32 = 1000;

fn run() -> u32 {
    let mut max = 0;
    let mut max_val = 0;
    for i in 1..MAX {
        // perform long division.
        let mut remainder = 1;
        let mut states = Vec::new();
        let repeat_len = loop {
            remainder *= 10;
            // let digit = remainder / i;
            remainder %= i;
            if let Some(n) = states.iter().position(|&x| x == remainder) {
                break states.len() - n;
            }
            states.push(remainder);
            if remainder == 0 {
                break 0;
            }
        };
        if repeat_len > max {
            max = repeat_len;
            max_val = i;
        }
    }
    max_val
}

fn main() {
    println!("{}", run());
}
