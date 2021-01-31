use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() {
    let s: Vec<usize> = (1..1000000).map(|e| e * (3 * e - 1) / 2).collect();
    let dict: HashSet<usize> = HashSet::from_iter(s.iter().map(|e| *e));
    let mut found = false;
    for i in 1..1000000 {
        for j in (0..i).rev() {
            let diff = s[i] - s[j];
            let sum = s[i] + s[j];
            if dict.contains(&diff) && dict.contains(&sum) {
                // println!("{} {}: {}", i, j, diff);
                println!("Answer: {}", diff);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}
