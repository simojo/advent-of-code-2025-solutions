use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = match File::open("day1.txt") {
        Err(_) => panic!("[ERROR] reading day1.txt.\n"),
        Ok(f) => f,
    };
    let mut buf: Vec<u8> = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let mut s: i32 = 0;
    let mut i = 0;
    const N_DIGITS: usize = 3;
    let mut d: [i32; N_DIGITS] = [0; N_DIGITS];
    let mut dial_inc: i32;
    let mut dial: i32 = 50;
    let mut n_times_dial_0 = 0;
    for c in buf {
        match c {
            b'L'          => s = -1,
            b'R'          => s =  1,
            b'\n' | b'\r' => {
                dial_inc = 0;
                for j in 0..i {
                    dial_inc += s * d[j] * (10_i32).pow((i as u32)-1);
                    i -= 1;
                }
                n_times_dial_0 += dial_inc.abs() / 100;
                let passed_zero: bool = dial != 0 && dial + (dial_inc % 100) != (dial + (dial_inc % 100)).rem_euclid(100);

                dial += dial_inc;
                dial = dial.rem_euclid(100);
                if dial == 0 || passed_zero {
                    n_times_dial_0 += 1;
                }
            },
            _             => {
                d[i] = (c - b'0') as i32;
                i += 1;
            }
        }
    }
    println!("Password: {}", n_times_dial_0);
}
