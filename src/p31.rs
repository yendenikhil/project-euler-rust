#[allow(dead_code)]
pub fn run() {
    let mut start = [1; 201];
    let coins = [2, 5, 10, 20, 50, 100, 200];
    for &x in &coins {
        for y in x..201 {
            start[y] += start[y - x];
        }
    }
    println!("2 pounds can be built in {} different ways.", start[200]);
}
