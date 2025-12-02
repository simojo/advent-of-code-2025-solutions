use std::{fs::File, io::Read};
use std::iter::*;
use std::str::{FromStr};
use std::thread;


fn is_symmetric(n: i64) -> bool {
    // if number is even length
    let n_as_str = n.to_string();
    let len = n_as_str.len();
    for divisor in 2..=len {
        // only check if divisor divides len
        if len % divisor != 0 {
            continue;
        }
        let mut prev = String::new();
        let mut curr = String::new();
        let mut all_equal = true;
        for i in 0..divisor {
            curr = n_as_str[(i * len/divisor)..((i + 1) * len/divisor)].into();
            // we just do first iteration to get curr for i = 0
            if i == 0 {
                prev = curr;
                continue;
            }
            if prev == curr {
                all_equal = true;
            } else {
                all_equal = false;
                break;
            }
            prev = curr;
        }
        if all_equal {
            return true;
        }
    }
    return false;
    // NOTE: This only applies for day 2, part 1
    if len % 2 == 0 {
        return n_as_str[0..(len/2)] == n_as_str[(len/2)..(len/2+len/2)];
    } else {
        return false;
    }
}

fn main() -> std::io::Result<()> {
    assert!(is_symmetric(33), "Test failed.");
    let mut f = File::open("day2.txt")?;
    let mut str: String = String::new();
    f.read_to_string(&mut str)?;
    let ranges: Vec<(&str, &str)> = str
        .strip_suffix("\n").unwrap()
        .split(',')
        .map(|s| (
            s.split('-').collect::<Vec<_>>()[0],
            s.split('-').collect::<Vec<_>>()[1],
        ))
        .collect();
    let mut n_symmetric_ids: i64 = 0;
    let mut sum_symmetric_ids: i64 = 0;
    for range in ranges {
        let r0 = i64::from_str(range.0).expect("Unable to parse number.");
        let r1 = i64::from_str(range.1).expect("Unable to parse number.");
        for n in r0..=r1 {
            if is_symmetric(n) {
                n_symmetric_ids += 1;
                sum_symmetric_ids += n;
            }
        }
    }
    println!("Symmetric IDs: {}", n_symmetric_ids);
    println!("Sum of Symmetric IDs: {}", sum_symmetric_ids);
    Ok(())
}
