impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = [0, 0, 0, 0, 0];
        for ch in text.chars() {
            match ch {
                'b' => cnt[0] += 1,
                'a' => cnt[1] += 1,
                'l' => cnt[2] += 1,
                'o' => cnt[3] += 1,
                'n' => cnt[4] += 1,
                _ => (),
            };
        }
        cnt[2] /= 2;
        cnt[3] /= 2;
        *cnt.iter().min().unwrap()
    }
}
