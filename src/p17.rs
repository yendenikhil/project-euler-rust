#[allow(dead_code)]
fn letters(num: usize) -> String {
    let singles = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let doubles = [
        "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let teens = [
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
    let and = "and";
    let hundred = "hundred";
    let mut ans = String::from("");
    let mut s = num;
    if s == 1000 {
        ans += "onethousand";
        s = 0;
    }
    if s >= 100 {
        ans += singles[s / 100];
        ans += hundred;
        s = s % 100;
        if s > 0 {
            ans += and;
        }
    }
    if s == 10 || s >= 20 {
        ans += doubles[s / 10 - 1];
        s = s % 10;
    }
    if s > 10 && s < 20 {
        ans += teens[s - 10];
        s = 0;
    }
    if s > 0 && s < 10 {
        ans += singles[s];
    }
    ans
}

#[allow(dead_code)]
pub fn run() {
    let mut count = 0;
    for i in 1..1001 {
        let s = letters(i);
        // println!("{}: {}", &s, s.len());
        count += s.len();
    }
    println!("1 to 1000 has {} letters.", count);
}
