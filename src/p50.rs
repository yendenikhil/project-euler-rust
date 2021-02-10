use std::collections::HashSet;

fn prime_sieve(size: usize) -> Vec<usize> {
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
    let mut ans = Vec::new();
    for (i, &x) in list.iter().enumerate() {
        if x {
            ans.push(i);
        }
    }
    ans
}

pub fn run() {
    let max = 1000000;
    let primes = prime_sieve(max);
    let s = primes.iter().collect::<HashSet<_>>();
    let mut sums = Vec::with_capacity(primes.len());
    let mut ans = 0;
    let mut counter = 0;
    sums.push(primes[0]);
    for i in 1..primes.len() {
        sums.push(primes[i] + sums[i - 1]);
    }
    for (i, x) in sums.iter().enumerate() {
        for (j, y) in sums[(i + 1..)].iter().enumerate() {
            if y - x > max {
                break;
            }
            // println!("x:{} y:{} i:{}, j:{}",x,y,i,j);
            if s.contains(&(y - x)) && j > counter {
                counter = j + 1;
                ans = y - x;
            }
        }
    }
    println!("Prime: {} with {} consequetive primes.", ans, counter);
}
