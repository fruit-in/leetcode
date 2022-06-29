impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let rings = rings.as_bytes();
        let mut rods = [0; 10];

        for i in (0..rings.len()).step_by(2) {
            match (rings[i], (rings[i + 1] - b'0') as usize) {
                (b'R', r) => rods[r] |= 1,
                (b'G', r) => rods[r] |= 2,
                (_, r) => rods[r] |= 4,
            }
        }

        rods.iter().filter(|&&r| r == 7).count() as i32
    }
}
