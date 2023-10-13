/// # Number Letter Counts
/// If the numbers 1 to 5 are written out in words:
/// one, two, three, four, five,
/// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
///
/// If all the numbers from 1 to 1000 (one thousand) inclusive
/// were written out in words, how many letters would be used?
///
/// NOTE: Do not count spaces or hyphens.
/// For example, 342 (three hundred and forty-two) contains 23 letters
/// and 115 (one hundred and fifteen) contains 20 letters.
/// The use of "and" when writing out numbers is
/// in compliance with British usage.

pub fn main() {
    const N: usize = 1000;
    let mut sum = 0;
    for i in 0..N {
        sum += count(i + 1);
    }
    assert_eq!(21124, sum);
}

pub fn count(n: usize) -> usize {
    let ones = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
    let tens = [3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
    let ty = [0, 3, 6, 6, 5, 5, 5, 7, 6, 6];
    let hundreds = [0, 10, 10, 12, 11, 11, 10, 12, 12, 11];
    const AND: usize = 3;
    let thousand = 11;
    let h = n / 100;
    let t = n % 100 / 10;
    let o = n % 10;
    if h == 10 {
        return thousand;
    }
    let mut tmp = hundreds[h];
    match t {
        1 => tmp += tens[o],
        0 => tmp += ones[o],
        _ => tmp += ty[t] + ones[o],
    }
    if h > 0 && (t > 0 || o > 0) {
        tmp += AND;
    }
    return tmp;
}
