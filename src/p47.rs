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
    for (i, x) in list.iter().enumerate() {
        if *x {
            ans.push(i);
        }
    }
    ans
}

fn prime_factors(num: usize, memo: &[usize]) -> Vec<usize> {
    let mut ans = Vec::new();
    for &x in memo {
        if x > num {
            break;
        }
        if num % x == 0 {
            ans.push(x);
        }
    }
    ans
}

pub fn run() {
    let max = 150_000;
    let primes = prime_sieve(max);
    let size = 4;
    for x in 14..(max - size) {
        let mut flag = true;
        for i in 0..size {
            let v = prime_factors(x + i, &primes);
            // println!("{}: {:?}", x + i, &v);
            if v.len() != size {
                flag = false;
                break;
            }
        }
        if flag {
            println!("Answer: {}", x);
            break;
        }
    }
}
