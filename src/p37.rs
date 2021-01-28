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

fn get_digits(num: u32, base: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push((n % base) as u8);
        n /= base;
    }
    digits
}

fn get_number(list: &[u8]) -> u32 {
    let mut num = 0;
    for (i, x) in list.iter().enumerate() {
        num += 10u32.pow(i as u32) * *x as u32;
    }
    num
}

pub fn run() {
    let primes = calc_primes(1_000_000);
    let mut count = 0;
    let mut sum = 0;
    for i in 11..1000000 {
        if primes[i] == true {
            let mut flag = true;
            let digits = get_digits(i as u32, 10);
            for x in 0..digits.len() {
                let n1 = get_number(&digits[..x + 1]);
                let n2 = get_number(&digits[x..]);
                if !primes[n1 as usize] || !primes[n2 as usize] {
                    flag = false;
                    break;
                }
            }
            if flag {
                count += 1;
                sum += i;
                if count == 11 {
                    break;
                }
            }
        }
    }
    println!("Answer: {}", sum);
}
