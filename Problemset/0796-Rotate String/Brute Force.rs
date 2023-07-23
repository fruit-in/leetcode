impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let mut a = a;

        if a.len() == b.len() {
            for _ in 0..=a.len() {
                if a == b {
                    return true;
                }

                let ch = a.remove(0);
                a.push(ch);
            }
        }

        false
    }
}
