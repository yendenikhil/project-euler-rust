pub mod p17;
pub mod p27;
pub mod p31;
pub mod p32;
pub mod p33;
pub mod p34;
pub mod p35;
pub mod p36;
pub mod p37;
pub mod p38;
pub mod p39;
pub mod p40;

pub fn run(num: u16) {
    match num {
        17 => p17::run(),
        27 => p27::run(),
        31 => p31::run(),
        32 => p32::run(),
        33 => p33::run(),
        34 => p34::run(),
        35 => p35::run(),
        36 => p36::run(),
        37 => p37::run(),
        38 => p38::run(),
        39 => p39::run(),
        40 => p40::run(),
        rest => println!("problem {} is not solved yet", rest),
    }
}
