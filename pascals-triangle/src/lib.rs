pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);

        for r in 1..=row_count as usize {
            let mut cols = vec![1u32; r];
            if r > 2 {
                for i in 1..r - 1 {
                    cols[i] = rows[r - 2][i - 1] + rows[r - 2][i];
                }
            }
            rows.push(cols);
        }

        PascalsTriangle { rows }
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }
}
