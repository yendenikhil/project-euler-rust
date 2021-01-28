fn calc_primes(num: usize) -> Vec<bool> {
    let mut list: Vec<bool> = vec![true; num + 1];
    list[1] = false;
    for x in 2..(num + 1) {
        if list[x] == true {
            for i in 2..(num + 2) {
                if i * x > num {
                    break;
                }
                list[x * i] = false;
            }
        }
    }
    list
}

fn check_number(num: usize, primes: &[bool]) -> bool {
    let digits: Vec<char> = num.to_string().chars().collect();
    let mut ans = true;
    for i in 0..digits.len() {
        let num = [&digits[i..], &digits[..i]]
            .concat()
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        ans = primes[num];
        if !ans {
            break;
        }
    }
    ans
}

pub fn run() {
    let primes = calc_primes(1000000);
    let mut ans = 0;
    for x in 2..1000001 {
        if check_number(x, &primes) {
            ans += 1;
        }
    }
    println!("Answer: {}", &ans);
}
