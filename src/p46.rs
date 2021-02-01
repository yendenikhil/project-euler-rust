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

pub fn run() {
    let max = 1_000_000;
    let seconds: HashSet<usize> = (1..(max / 2)).map(|e| 2 * e * e).collect();
    let primes = prime_sieve(max);
    let mut prime_series = Vec::new();
    for (i, &x) in primes.iter().enumerate() {
        if x {
            prime_series.push(i);
        }
    }

    let mut x = 15;
    loop {
        if x >= max {
            break;
        }
        if !primes[x] {
            let mut m = false;
            for p in &prime_series {
                if x < *p {
                    break;
                }
                if seconds.contains(&(x - p)) {
                    m = true;
                    break;
                }
            }
            if !m {
                break;
            }
        }
        x += 2;
    }
    println!("Answer: {}", x);
}
