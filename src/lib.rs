pub mod p17;
pub mod p27;
pub mod p31;
pub mod p32;
pub mod p33;

pub fn run(num: u16) {
    match num {
        17 => p17::run(),
        27 => p27::run(),
        31 => p31::run(),
        32 => p32::run(),
        33 => p33::run(),
        rest => println!("problem {} is not solved yet", rest),
    }
}