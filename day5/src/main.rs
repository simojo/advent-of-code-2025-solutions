use std::{fs::File, io::Read};
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let mut ids: HashMap<u64, bool> = HashMap::new();
    let mut f = File::open("day5.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let file_parsed: Vec<Vec<u64>> = buf
        .trim()
        .split('\n')
        .filter(|line| line.len() > 1)
        .map(|line| line
            .split('-')
            .map(|num_str| u64::from_str(num_str).unwrap())
            .collect())
        .collect();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    file_parsed
        .iter()
        .for_each(|nums: &Vec<u64>| {
            if nums.len() == 1 {
                ids.insert(*nums.first().unwrap(), false);
            } if nums.len() == 2 {
                ranges.push((nums[0], nums[1]));
            }
        });

    // merge ranges
    loop {
        let mut index_to_remove: Option<usize> = None;
        'ranges_orig: for i in 0..ranges.len() {
            let (r0_orig, r1_orig) = ranges[i];
            let mut remove_original = false;
            for j in 0..ranges.len() {
                if i == j {
                    continue;
                }
                let (r0, r1) = &mut ranges[j];
                // our range surrounds existing
                if r0_orig <= *r0 && r1_orig >= *r1 {
                    // replace range with our range
                    *r0 = r0_orig;
                    *r1 = r1_orig;
                    remove_original = true;
                }
                // our range is inside existing
                else if r0_orig >= *r0 && r1_orig <= *r1 {
                    // do nothing!
                    remove_original = true;
                }
                // our range straddles existing upper bound
                else if r0_orig <= *r1 && r1_orig >= *r1 {
                    // make existing upper bound our upper bound
                    *r1 = r1_orig;
                    remove_original = true;
                }
                // our range straddles existing lower bound
                else if r0_orig <= *r0 && r1_orig >= *r0 {
                    // make existing lower bound our lower bound
                    *r0 = r0_orig;
                    remove_original = true;
                }
                if remove_original {
                    index_to_remove = Some(i);
                    break 'ranges_orig;
                }
            }
        }
        // sort by reverse to not break removing by index
        match index_to_remove {
            Some(i) => ranges.remove(i),
            None => break,
        };
    }
    for (r0, r1) in ranges.iter() {
        for (id, is_fresh) in ids.iter_mut() {
            if *r0 <= *id && *id <= *r1 {
                *is_fresh = true;
            }
        }
    }
    let valid_ids_p1: Vec<u64> = ids.iter()
        .filter(|(_k, v)| **v)
        .map(|(k, _v)| *k)
        .collect();
    let valid_ids_p2: u64 = ranges
        .iter()
        .fold(0, |acc, (r0, r1)| acc + r1-r0+1);

    println!("Part 1 answer: {:?}", valid_ids_p1.len());
    println!("Part 2 answer: {:?}", valid_ids_p2);

    println!("Done.");
}
