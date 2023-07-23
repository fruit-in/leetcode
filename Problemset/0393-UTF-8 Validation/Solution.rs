impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut data = data.into_iter();

        while let Some(byte) = data.next() {
            match Self::leading_ones(byte as u8) {
                0 => (),
                n if n > 1 && n < 5 => {
                    for _ in 1..n {
                        if Self::leading_ones(data.next().unwrap_or(0) as u8) != 1 {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }

        true
    }

    fn leading_ones(n: u8) -> u32 {
        for i in 0..8 {
            if n & (128 >> i) == 0 {
                return i;
            }
        }

        8
    }
}
