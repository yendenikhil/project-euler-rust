use std::collections::HashMap;

fn sieve_squars(num: usize) -> HashMap<usize, usize> {
    let mut list = HashMap::new();
    for i in 1..(num + 1) {
        list.insert(i * i, i);
    }
    let mut sq = HashMap::new();
    for i in 1..(num + 1) {
        for j in 1..(num + 1) {
            let key = i * i + j * j;
            if list.contains_key(&key) {
                let p = i + j + list.get(&key).unwrap();
                let count = sq.entry(p).or_insert(0);
                *count += 1;
            }
        }
    }
    sq
}
pub fn run() {
    let mut pp = 0;
    let mut cc = 0;
    let sq = sieve_squars(1000);
    for (k, v) in sq {
        if v > cc && k <= 1000 {
            cc = v;
            pp = k;
        }
    }

    println!("Answer: {}", pp);
}
