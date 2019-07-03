pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    use std::collections::btree_set::BTreeSet;

    let mut multiples = BTreeSet::new();

    for n in 1..limit {
        for &f in factors {
            multiples.insert(f * n);
        }
    }

    multiples
        .iter()
        .take_while(|&&x| x < limit)
        .sum()
}
