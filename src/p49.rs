use std::collections::HashMap;
use std::collections::HashSet;

fn prime_sieve(size: usize) -> Vec<bool> {
    let mut list = vec![true; size + 1];
    list[0] = false;
    list[1] = false;
    for i in 2..(size + 1) {
        if list[i] {
            for j in 2..(size / i + 1) {
                list[i * j] = false
            }
        }
    }
    list
}
fn perm(list: &[u8]) -> Vec<Vec<u8>> {
    if list.len() < 2 {
        return vec![vec![list[0]]];
    }
    let mut ans = Vec::new();
    for (i, &x) in list.iter().enumerate() {
        let mut rem: Vec<u8> = list.to_vec();
        rem.remove(i);
        let sub = perm(&rem);
        for ss in &sub {
            let mut sub = vec![x];
            for s in ss {
                sub.push(*s);
            }
            ans.push(sub);
        }
    }
    ans
}

fn digits_to_number(list: &[u8]) -> usize {
    list.iter().enumerate().fold(0_usize, |ans, (i, &x)| {
        ans + x as usize * 10_usize.pow(i as u32)
    })
}

fn is_match(list: &[usize], primes: &[bool]) -> Option<String> {
    let mut data = list.iter().collect::<Vec<_>>();
    let mut cache = HashMap::new();
    data.sort();
    for (i, &x) in data.iter().enumerate() {
        for &y in &data[i + 1..] {
            let diff = y - x;
            let val = cache.entry(diff).or_insert(Vec::new());
            if !val.contains(&(x, y)) {
                val.push((x, y));
            }
        }
    }

    for (_, v) in cache {
        if v.len() == 2 {
            let (&a, &b) = v[0];
            let (&bb, &c) = v[1];
            if b == bb && primes[a] && primes[b] && primes[c] {
                let s = format!("{}{}{}", a, b, c);
                return Some(s);
            }
        }
    }
    None
}

pub fn run() {
    let primes = prime_sieve(10000);
    let ans = (1000..9999)
        .map(|d| {
            d.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .map(|s| perm(&s))
        .map(|v| v.iter().map(|e| digits_to_number(&e)).collect::<Vec<_>>())
        .map(|v| is_match(&v, &primes))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<HashSet<_>>();
    for x in ans {
        if x.len() == 12 && !x.contains("1487") {
            println!("Answer: {}", x);
        }
    }
}
