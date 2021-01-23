pub mod p17;
pub mod p27;
pub mod p31;

pub fn run(num: u16) {
    match num {
        17 => p17::run(),
        27 => p27::run(),
        31 => p31::run(),
        rest => println!("problem {} is not solved yet", rest),
    }
}
