pub fn primes_up_to(n: u64) -> Vec<u64> {
    let mut states = vec![true; n as usize + 1];
    let sqrt = f64::sqrt(n as f64) as usize;

    for i in 2..=sqrt {
        if states[i] {
            let mut j = i * i;
            while j <= n as usize {
                states[j] = false;
                j += i;
            }
        }
    }

    states.iter()
        .enumerate()
        .filter(|(_,&v)| v)
        .map(|x| x.0 as u64)
        .skip(2)
        .collect()
}
