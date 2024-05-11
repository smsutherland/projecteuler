const NAMES_RAW: &str = include_str!("../data/0022_names.txt");

fn name_score((i, s): (usize, &str)) -> u64 {
    let s: u64 = s.as_bytes().iter().map(|c| (c - (b'A' - 1)) as u64).sum();
    s * (i + 1) as u64
}

fn run() -> u64 {
    let mut names: Vec<_> = NAMES_RAW.split(',').map(|s| &s[1..s.len() - 1]).collect();
    names.sort_unstable();
    names.iter().copied().enumerate().map(name_score).sum()
}

fn main() {
    println!("{}", run());
}
