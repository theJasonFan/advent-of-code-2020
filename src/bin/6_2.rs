use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(args.last().unwrap()).unwrap();

    let mut sum = 0;
    let mut bv: u32 = !0;
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                sum += bv.count_ones();
                bv = !0;
            } else {
                bv &= answer_to_bv(&line);
            }
        }
    }
    sum += bv.count_ones();

    println!("{}", sum);
}

fn answer_to_bv(s: &str) -> u32 {
    let mut bv = 0u32;
    for c in s.chars() {
        bv = set_bit((c as u32) - ('a' as u32), bv)
    }
    bv
}

fn set_bit(i: u32, bv: u32) -> u32 {
    bv | (1 << i)
}

//Rust by example: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(0, 0), 1);
        assert_eq!(set_bit(1, 0), 2);
        assert_eq!(set_bit(3, 1), 9);
    }
}
