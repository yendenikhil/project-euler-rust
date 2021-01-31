use std::fs;

fn read_file(name: &str) -> Vec<String> {
    let contents = fs::read_to_string(name).expect("Failed to read file");
    contents
        .trim()
        .split(',')
        .map(|s| s[1..s.len() - 1].to_string())
        .collect()
}

fn sum_of_nums(max: usize) -> Vec<usize> {
    (1..)
        .map(|e| e * (e + 1) / 2)
        .take_while(|e| *e <= max)
        .collect()
}
pub fn run() {
    let name = "./src/p042_words.txt";
    let dict: Vec<char> = "0ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let lines = read_file(name);
    // max length of the word is 14 so max sum would be 26 * 14 = 364
    let series = sum_of_nums(364);
    let mut ans = 0;
    for line in lines {
        let sum = line
            .chars()
            .map(|e| dict.iter().position(|&ee| ee == e).unwrap())
            .sum();
        if let Some(_) = series.iter().position(|&ee| ee == sum) {
            ans += 1;
        };
    }
    println!("Answer: {}", ans);
}
