impl Solution {
    pub fn sum_game(num: String) -> bool {
        let num = num.as_bytes();
        let mut diff = 0;
        let mut diff_mask = 0;

        for i in 0..num.len() / 2 {
            if num[i] != b'?' {
                diff += (num[i] - b'0') as i32;
            } else {
                diff_mask -= 1;
            }
        }
        for i in num.len() / 2..num.len() {
            if num[i] != b'?' {
                diff -= (num[i] - b'0') as i32;
            } else {
                diff_mask += 1;
            }
        }

        diff_mask % 2 != 0 || diff_mask / 2 * 9 != diff
    }
}
