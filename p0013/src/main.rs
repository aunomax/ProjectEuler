/// # Large Sum
///
/// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
///
///
///
///
///
use std::fs::File;
use std::io::{self, BufRead};

use std::path::Path;

fn main() {
    let mut sum = String::new();
    if let Ok(lines) = read_lines("1500digits.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if sum.len() > ip.len() {
                    sum = large_sum(sum, ip).clone();
                } else {
                    sum = large_sum(ip, sum).clone();
                }
            }
        }
    }
    sum.truncate(10);
    assert_eq!("5537376230", sum);
}

pub fn large_sum(mut sum: String, mut ip: String) -> String {
    let mut tmp = String::new();
    let mut b = 0;
    for _i in 0..sum.len() {
        match ip.pop() {
            Some(c) => {
                let s = sum.pop().unwrap();
                let s1 = s as u8 - '0' as u8 + c as u8 - '0' as u8 + b;
                b = s1 / 10;
                tmp.push((s1 % 10 + '0' as u8) as char);
            }
            None => {
                let s = sum.pop().unwrap();
                let s1 = s as u8 - '0' as u8 + b;
                b = s1 / 10;
                tmp.push((s1 % 10 + '0' as u8) as char);
            }
        }
    }
    if b != 0 {
        tmp.push((b + '0' as u8) as char);
    }
    let mut result = String::new();
    for _i in 0..tmp.len() {
        if let Some(c) = tmp.pop() {
            result.push(c as char);
        }
    }
    return result;
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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
    fn test_large_sum() {
        assert_eq!(
            large_sum(String::from("99"), String::from("1")),
            String::from("100")
        );
    }
}
