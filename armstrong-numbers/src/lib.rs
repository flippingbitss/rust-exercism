pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut digits = Vec::new();

    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);

    let e = digits.len() as u32;

    num == digits.iter().map(|&d| u32::pow(d, e)).sum()
}
