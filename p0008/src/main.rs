/// # Largest Product in a Series
///
/// The four adjacent digits in the 1000-digit number that have the greatest product are
/// 9*9*8*9=5832.
///
///  73167176531330624919225119674426574742355349194934
///  96983520312774506326239578318016984801869478851843
///  85861560789112949495459501737958331952853208805511
///  12540698747158523863050715693290963295227443043557
///  66896648950445244523161731856403098711121722383113
///  62229893423380308135336276614282806444486645238749
///  30358907296290491560440772390713810515859307960866
///  70172427121883998797908792274921901699720888093776
///  65727333001053367881220235421809751254540594752243
///  52584907711670556013604839586446706324415722155397
///  53697817977846174064955149290862569321978468622482
///  83972241375657056057490261407972968652414535100474
///  82166370484403199890008895243450658541227588666881
///  16427171479924442928230863465674813919123162824586
///  17866458359124566529476545682848912883142607690042
///  24219022671055626321111109370544217506941658960408
///  07198403850962455444362981230987879927244284909188
///  84580156166097919133875499200524063689912560717606
///  05886116467109405077541002256983155200055935729725
///  71636269561882670428252483600823257530420752963450
///
///  Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?
///
///
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const N: usize = 13;
    let mut numbers = String::new();
    if let Ok(lines) = read_lines("1000-digit.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers.push_str(&ip);
            }
        }
    }
    //let reader = BufReader<BufRead>::new(file);
    //let digits = numbers.chars().to_digit(10)?;
    let chars = numbers
        .chars()
        .map(|c| c as u64 - '0' as u64)
        .collect::<Vec<u64>>();
    let mut max: u64 = 0;
    for i in 0..chars.len() - N {
        let mut tmp: u64 = 1;
        for j in i..i + N {
            tmp *= chars[j];
        }
        if tmp > max {
            max = tmp;
        }
    }
    assert_eq!(23514624000, max);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
