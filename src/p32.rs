use std::collections::HashSet;

fn is_pandigital(a: usize, b: usize, c: usize) -> bool {
    let s = format!("{}{}{}", a, b, c);
    if s.len() == 9 {
        let mut c: Vec<char> = s.chars().collect();
        c.sort();
        let cc: String = c.into_iter().collect();
        if cc == "123456789" {
            return true;
        }
    }
    false
}

pub fn run() {
    let mut s: HashSet<usize> = HashSet::new();
    for a in 1..99 {
        for b in 123..9876 {
            let flag = is_pandigital(a, b, a * b);
            if flag {
                // println!("{} x {} = {}", a, b, a * b);
                s.insert(a * b);
            }
        }
    }
    let ans: usize = s.iter().sum();
    println!("Answer: {}", ans);
}
