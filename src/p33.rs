fn is_digit_cancelling(num: usize, den: usize) -> bool {
    let nums: Vec<f64> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as f64)
        .collect();
    let dens: Vec<f64> = den
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as f64)
        .collect();
    let res: f64 = num as f64 / den as f64;
    let n1 = nums[0];
    let n2 = nums[1];
    let d1 = dens[0];
    let d2 = dens[1];
    if n2 == d1 {
        return res == n1 / d2;
    }
    false
}

pub fn run() {
    let mut ans = 1.0;
    for n in 10..100 {
        for d in (n + 1)..100 {
            if is_digit_cancelling(n, d) {
                // println!("{}/{}", n, d);
                ans *= d as f64 / n as f64;
            }
        }
    }
    println!("Answer: {}", ans);
}
