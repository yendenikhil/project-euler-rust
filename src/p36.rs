fn get_digits(num: u32, base: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push((n % base) as u8);
        n /= base;
    }
    digits
}
fn is_palindrom(list: &[u8]) -> bool {
    let max = list.len() / 2 + 1;
    let l = list.len();
    for i in 0..max {
        if list[i] != list[l - 1 - i] {
            return false;
        }
    }
    true
}

pub fn run() {
    let mut sum = 0;
    for i in 1..1000000 {
        let bin = is_palindrom(&get_digits(i, 2));
        let dec = is_palindrom(&get_digits(i, 10));
        if bin && dec {
            sum += i;
        }
    }
    println!("Answer: {}", sum);
}
