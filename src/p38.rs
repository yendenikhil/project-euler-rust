use std::cmp;

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    let mut cc: Vec<char> = s.chars().collect();
    cc.sort();
    let c: String = cc.into_iter().collect();
    c == "123456789"
}

pub fn run() {
    let mut ans = 0;
    for x in 1..10000 {
        for n in 2..7 {
            let mut s = String::from("");
            for i in 1..(n + 1) {
                s = format!("{}{}", s, x * i);
                if s.len() >= 9 {
                    break;
                }
            }
            if is_pandigital(&s) {
                let xx = s.parse::<usize>().unwrap();
                ans = cmp::max(ans, xx);
            }
        }
    }
    println!("Answer: {}", ans);
}
