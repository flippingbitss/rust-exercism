use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A','C','G','T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        if VALID_NUCLEOTIDES.contains(&c) {
            if c == nucleotide {
                count += 1;
            }
        } else {
            return Err(c);
        }
    }
    return Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(c) = dna.chars().find(|c| !VALID_NUCLEOTIDES.contains(c)) {
        return Err(c);
    }
    let mut map = HashMap::new();
    for &c in VALID_NUCLEOTIDES.iter() {
        let count = count(c, dna)?;
        map.insert(c, count);
    }
    Ok(map)
}
