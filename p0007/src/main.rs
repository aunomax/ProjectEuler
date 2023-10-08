fn main() {
    let mut count = 10001;
    let mut prime = 0;
    while count != 0 {
        prime +=1;
        if is_prime(prime) {
            count -= 1;
        }
    }
    assert_eq!(104743, prime);
}


fn is_prime(n:u64) -> bool {
    if n <= 1 {
        return false
    }
    let mut tmp:u64 = 2;
    while tmp.pow(2) <= n {
        if n%tmp == 0 {
            return false
        }
        tmp += 1;
    }
    return true
}
