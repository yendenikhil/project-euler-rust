pub fn run() {
    let mut t: usize = 286;
    let mut p: usize = 166;
    let mut h: usize = 144;
    loop {
        if t % 10000 == 0 {
            println!("{} {} {}", t, p, h);
        }
        let tt = t * (t + 1) / 2;
        let pp = p * (3 * p - 1) / 2;
        let hh = h * (2 * h - 1);

        if tt == pp && pp == hh {
            // println!("{} {} {}: {}", t, p, h, hh);
            println!("Answer: {}", hh);
            break;
        }

        if tt < pp || tt < hh {
            t += 1;
        }
        if pp < tt || pp < hh {
            p += 1;
        }
        if hh < pp || hh < tt {
            h += 1;
        }
    }
}
