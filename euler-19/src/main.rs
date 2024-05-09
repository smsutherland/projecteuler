const SUNDAY: u32 = 6;

const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

fn main() {
    let mut year = 1900;
    let mut month = 0;
    let mut day = 0;
    let mut total = 0;
    while year < 2001 {
        day += DAYS[month];
        if month == 1 && is_leap_year(year) {
            day += 1;
        }
        if day % 7 == SUNDAY && year >= 1901 {
            total += 1;
        }
        month += 1;
        if month == 12 {
            month = 0;
            year += 1;
        }
    }
    println!("{total}");
}
