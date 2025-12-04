use std::io::*;
use std::fs::File;

fn xy_to_index((x,y): (i32, i32), linelen: u16) -> u16 {
    (y * (linelen as i32) + x) as u16
}

fn index_to_xy(i: usize, linelen: u16) -> (u16, u16) {
    (
        (i as u16) % linelen,
        (i as u16) / linelen,
    )
}

/*
 * Look at the (clipped) surrounding slots to see if any of them are scrolls
 */
fn sum_of_surrounding(buf: &Vec<u8>, i: usize, linelen: u16, nlines: u16) -> u32 {
    let (x,y) = index_to_xy(i, linelen);
    let mut sum: u32 = 0;
    let ylb = (0).max((y as i32)-1) as u16;
    let yub = (nlines-1).min(y+1);
    let xlb = (0).max((x as i32)-1) as u16;
    let xub = (linelen-1).min(x+1);
    for yi in ylb..=yub {
        for xi in xlb..=xub {
            let val = buf[xy_to_index((xi as i32,yi as i32), linelen) as usize] as u32;
            if xi != x || yi != y {
                sum += val;
            }
        }
    }
    sum
}

fn main() {
    let mut f = File::open("day4.txt").expect("[ERROR] day4.txt");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("[ERROR] reading day4.txt");
    let linelen = buf.find('\n')
        .expect("[ERROR] Unable to find line length.") as u16;
    let nlines = buf.matches('\n').collect::<Vec<&str>>().len() as u16;
    println!("linelen {} nlines {}", linelen, nlines);
    let mut n_removable: i32 = 0;
    let mut buf_as_bytes = buf.as_bytes().to_vec();
    buf_as_bytes = buf_as_bytes
        .iter()
        .filter(|&it| *it == ('@' as u8) || *it == ('.' as u8))
        .map(|&it| it)
        .collect();
    loop {
        let mut buf_as_10s = buf_as_bytes
            .iter()
            .map(|it| if *it == ('@' as u8) {1} else {0}).collect();
        let n_removable_this_time = buf_as_bytes
            .iter()
            .enumerate()
            .fold(0, |acc, (i, _it)| {
                let mut acc_res = 0;
                if *_it == '@' as u8 && sum_of_surrounding(&buf_as_10s, i, linelen, nlines) < 4 {
                    // we can remove this one
                    acc_res += 1;
                    buf_as_10s[i] = 0;
                }
                acc + acc_res
            });
        // now that we're done, let's update buf according to buf_as_10s
        buf_as_bytes = buf_as_bytes
            .iter()
            .enumerate()
            .map(|(i, _it)| if buf_as_10s[i] == 1 {'@' as u8} else {'.' as u8})
            .collect();
        n_removable += n_removable_this_time;
        if n_removable_this_time == 0 {
            break;
        }
        println!("{}", n_removable_this_time);
    }
    println!("{}", n_removable);
}
