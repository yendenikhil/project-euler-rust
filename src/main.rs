use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let p = Args::new(&args);
    p.run();
}

struct Args {
    pub num: u16,
}
impl Args {
    fn new(args: &[String]) -> Args {
        let default_num = 27;
        let def_str = &default_num.to_string();
        let param = match args.get(1) {
            Some(v) => v,
            None => def_str,
        };
        let num: u16 = param.parse::<u16>().unwrap();
        Args { num }
    }

    fn run(&self) {
        let start = Instant::now();
        euler::run(self.num);
        let end = Instant::now();
        println!("\ntime: {:?}", end.duration_since(start));
    }
}
