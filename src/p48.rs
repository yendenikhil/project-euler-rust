pub fn run() {
    let max: usize = 1000;
    let modulo: usize = 10_000_000_000;
    let mut sum: usize = 0;
    for x in 1..(max + 1) {
        let mut num = x;
        for _ in 1..x {
            num *= x;
            num %= modulo;
        }
        sum += num;
        sum %= modulo;
    }
    println!("Answer: {}", sum);
}
