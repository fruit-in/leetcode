impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut amt = 0;
        let mut cnt = 0;

        for ch in s.chars() {
            if ch == 'R' {
                cnt += 1;
            } else if ch == 'L' {
                cnt -= 1;
            }

            if cnt == 0 {
                amt += 1;
            }
        }

        amt
    }
}
