pub fn factors(n: u64) -> Vec<u64> {
    let mut f = 2;
    let mut num = n;
    let mut factors = Vec::new();

    while f * f <= num {
        if num % f == 0 {
            num /= f;
            factors.push(f);
            continue;
        }
        f += 1;
    }
    factors.push(num);
    factors
}
