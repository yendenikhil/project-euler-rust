// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

fn letters(num: usize) -> String {
    let singles = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let doubles = ["ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let teens = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
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

pub fn run() {
    let mut count = 0;
    for i in 1..1001 {
        let s = letters(i);
        // println!("{}: {}", &s, s.len());
        count += s.len();
    }
    println!("1 to 1000 has {} letters.", count);
}
