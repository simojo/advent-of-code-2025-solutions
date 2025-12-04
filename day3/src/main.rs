use std::io::*;
use std::fs::File;

const N_BANKS: u32 = 12;

fn main() {
    let mut f = File::open("day3.txt").expect("[ERROR] day3.txt");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("[ERROR] reading day3.txt");
    let res = buf
        .trim()
        .split('\n')
        .fold(0_u64, |acc, line| {

        let mut i0: usize = 0;
        let mut i1: usize;
        let mut d: [u8; N_BANKS as usize] = [0; N_BANKS as usize];
        let linelen = line.len();
        for i in 0..N_BANKS {
            i1 = linelen-(N_BANKS as usize - (i as usize));
            d[i as usize] = line[i0..=i1]
                .as_bytes()
                .iter()
                .fold(0, |a, it| a.max(*it));
            for ii in 0..=(i1-i0) {
                if line[i0..=i1].as_bytes()[ii] == d[i as usize] {
                    i0 += ii + 1;
                    break;
                }
            }
        }
        acc + d
            .iter()
            .enumerate()
            .fold(0_u64, |acc, (i, di)|
                acc + ((di - '0' as u8) as u64) * (10_u64).pow(N_BANKS-1-(i as u32))
            )
    });
    println!("{}", res);
}
