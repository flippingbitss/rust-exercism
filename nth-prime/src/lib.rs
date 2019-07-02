pub fn nth(n: u32) -> u32 {
    (2..)
        .into_iter()
        .filter(|&x| is_prime(x))
        .take(n as usize + 1)
        .last()
        .unwrap()
}


fn is_prime(n: u32) -> bool {
    let mut m = 2;
    while m * m <= n {
        if n % m == 0 {
            return false;
        }
        m += 1;
    }
    true
}