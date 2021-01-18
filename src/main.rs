use std::env;

mod problems;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let num: u16 = args[1].parse().unwrap();
        problems::run(num)
    } else {
        problems::run(27);
    }
}
