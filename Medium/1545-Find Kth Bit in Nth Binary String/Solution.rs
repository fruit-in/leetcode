impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut bits = vec![false];

        loop {
            if let Some(&b) = bits.get(k as usize - 1) {
                return char::from(b as u8 + b'0');
            }

            let mut x = bits.clone().iter().map(|&b| !b).rev().collect::<Vec<_>>();
            bits.push(true);
            bits.append(&mut x);
        }
    }
}
