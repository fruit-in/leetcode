impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let x = x as u8 + b'0';
        let mut n = n.into_bytes();

        if n[0] != b'-' {
            for i in 0..=n.len() {
                if i == n.len() || x > n[i] {
                    n.insert(i, x);
                    break;
                }
            }
        } else {
            for i in 1..=n.len() {
                if i == n.len() || x < n[i] {
                    n.insert(i, x);
                    break;
                }
            }
        }

        String::from_utf8(n).unwrap()
    }
}
