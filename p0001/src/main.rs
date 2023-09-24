fn main() {
    let mut sum = 0;
    for n in 1..1000 {
        sum += ismod3or5(n);
    }
//  println!("Hello, world!");
    println!("{}", sum);
}

fn ismod3or5(n: i32) -> i32 {
    if n%3 * n%5 == 0 {
        return n
    }
    return 0
}
