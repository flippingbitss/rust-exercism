pub struct Triangle {
    is_equilateral: bool,
    is_scalene: bool,
    is_isosceles: bool,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // check for triangle inequality
        for (i, &side) in sides.iter().enumerate() {
            if sides
                .iter()
                .enumerate()
                .filter_map(|(j, s)| if j != i { Some(*s) } else { None })
                .sum::<u64>()
                <= side
            {
                return None;
            }
        }

        let [side_a, side_b, side_c] = sides;

        let is_equilateral = side_a == side_b && side_b == side_c;
        let is_isosceles = side_a == side_b || side_b == side_c || side_c == side_a;

        Some(Triangle {
            is_equilateral,
            is_isosceles,
            is_scalene: !is_isosceles,
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.is_equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.is_scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.is_isosceles
    }
}
