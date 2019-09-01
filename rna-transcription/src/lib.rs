#[derive(Debug, PartialEq)]
pub struct DNA {
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    value: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, n) in dna.char_indices() {
            match n {
                'A' | 'C' | 'G' | 'T' => {}
                _ => return Err(i),
            }
        }
        Ok(DNA {
            value: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let value = self
            .value
            .chars()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!()
            })
            .collect();

        RNA { value }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, n) in rna.char_indices() {
            match n {
                'A' | 'C' | 'G' | 'U' => {}
                _ => return Err(i),
            }
        }
        Ok(RNA {
            value: rna.to_string(),
        })
    }
}
