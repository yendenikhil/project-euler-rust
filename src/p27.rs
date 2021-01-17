use std::collections::HashMap;

#[allow(dead_code)]
fn is_prime(num: i32, memo: &mut HashMap<i32, bool>) -> bool {
    let mut prime = true;
    if memo.contains_key(&num) {
        prime = *memo.get(&num).expect("SOmething went wrong!");
    } else {
        if num < 3 {
            prime = false;
        } else {
            for i in (3..num).step_by(2) {
                if num % i == 0 {
                    prime = false;
                    break;
                }
            }
        }
        memo.insert(num, prime);
    }
    prime
}

#[allow(dead_code)]
fn solve(a: i32, b: i32, memo: &mut HashMap<i32, bool>) -> i32 {
    // n^2 + a * n + b
    let mut n = 0;
    if is_prime(b, memo) && is_prime(1 + a + b, memo) {
        loop {
            let q = n * n + a * n + b;
            // println!("{}: {}: {}", n, q, is_prime(q));
            if !is_prime(q, memo) {
                break;
            }
            n += 1;
        }
    }
    n
}

#[allow(dead_code)]
pub fn run() {
    let mut memo: HashMap<i32, bool> = HashMap::new();
    let mut max = 0;
    let mut ans = 0;
    for a in -1000..1001 {
        for b in -1000..1001 {
            let num = solve(a, b, &mut memo);
            if max < num {
                // println!("a: {}, b: {}: {}", a, b, num);
                max = num;
                ans = a * b;
            }
        }
    }
    println!("Ans: {}", ans);
}
