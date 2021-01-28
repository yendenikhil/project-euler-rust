use std::collections::HashMap;

fn factorial(num: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if memo.contains_key(&num) {
        return *memo.get(&num).unwrap()
    }
    if num > 0 {
        let mut ans: u32 = 1;
        for i in 1..(num + 1) {
            ans *= i;
        }
        memo.insert(num, ans);
        return ans;
    } else {
        1
    }
}

fn is_match(num: usize, memo: &mut HashMap<u32, u32>) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|e| e.to_digit(10).unwrap())
        .collect();
    let sum = digits.iter().map(|e| factorial(*e, memo)).fold(0, |a, b| a + b);
    // println!("{} {}", num, sum);
    num == sum as usize
}

pub fn run() {
    //println!("{}", is_match(145));
    let mut memo: HashMap<u32, u32> = HashMap::new();
    let mut ans = 0;
    for i in 10..99999 {
        if is_match(i, &mut memo) {
            // println!(">>>> {}", i);
            ans += i;
        }
    }
    println!("Answer: {}", ans);
}
