impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..=(n / 2) {
            let b = n - a;
            let ab_str = a.to_string() + &b.to_string();

            if ab_str.bytes().all(|x| x != b'0') {
                return vec![a, b];
            }
        }

        Vec::new()
    }
}
