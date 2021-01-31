fn perm(list: &[u8]) -> Vec<Vec<u8>> {
    if list.len() < 2 {
        return vec![vec![list[0]]];
    }
    let mut ans = Vec::new();
    for x in list {
        let rem: Vec<u8> = list.iter().filter(|e| *e != x).map(|e| *e).collect();
        let sub = perm(&rem);
        for ss in &sub {
            let mut sub = vec![*x];
            for s in ss {
                sub.push(*s);
            }
            ans.push(sub);
        }
    }
    ans
}

fn digitis_to_usize(list: &[u8]) -> usize {
    list.iter()
        .map(|e| e.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

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

pub fn run() {
    let primes = prime_sieve(7654321);
    let ans = perm(&vec![1, 2, 3, 4, 5, 6, 7])
        .iter()
        .map(|v| digitis_to_usize(&v))
        .filter(|e| primes[*e])
        .max()
        .unwrap();
    println!("Answer: {}", ans);
}
