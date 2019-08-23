impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut cnt = 0;
        let mut flag = false;
        for c in s.chars() {
            if c == ' ' {
                flag = true;
            } else if flag {
                cnt = 1;
                flag = false;
            } else {
                cnt += 1;
            }
        }
        cnt
    }
}
