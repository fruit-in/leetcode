impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let m = m as i64;
        let mut x = 0;
        let mut div = vec![0; word.len()];

        for (i, digit) in word.bytes().enumerate() {
            x = (x * 10 + (digit - b'0') as i64) % m;
            div[i] = 0.max(1 - x as i32);
        }

        div
    }
}
