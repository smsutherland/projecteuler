const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const FIRST20: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

fn to_british_english(num: u32) -> String {
    if num < 20 {
        return FIRST20[num as usize].into();
    }
    if num < 100 {
        let tens = num / 10;
        let remainder = num % 10;
        let start = TENS[tens as usize].to_owned();
        if remainder != 0 {
            return start + &to_british_english(remainder);
        } else {
            return start;
        }
    }
    if num < 1000 {
        let hundreds = num / 100;
        let remainder = num % 100;
        let start = to_british_english(hundreds) + "hundred";
        if remainder != 0 {
            return start + "and" + &to_british_english(remainder);
        } else {
            return start;
        }
    }
    "onethousand".into()
}

fn run() -> usize {
    let mut sum = 0;
    for i in 1..=1000 {
        let s = to_british_english(i);
        sum += s.len();
    }
    sum
}

fn main() {
    println!("{}", run());
}
