fn get_number(pos: usize) -> usize {
    let dd = [0, 9, 90, 900, 9000, 90000, 900000];
    let mut d = pos;
    for (i, x) in dd.iter().enumerate() {
        if d <= i * x {
            let start = 10_usize.pow((i - 1) as u32);
            let add = (d - 1) / i;
            let rem = (d - 1 + i) % i;
            let num = start + add;
            let cc = num.to_string().chars().collect::<Vec<char>>();
            let ans = cc[rem].to_string().parse::<usize>().unwrap();
            // println!("pos: {}, num: {}, rem:{}, ans: {}", pos, num, rem, ans);
            return ans;
        } else {
            d -= i * *x;
        }
    }
    println!("Problem");
    0
}
pub fn run() {
    let ans = (0..7)
        .map(|e| 10_usize.pow(e as u32))
        .map(|e| get_number(e))
        .fold(1, |a, b| a * b);
    println!("Answer: {}", ans);
}
