impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut s = s.bytes().peekable();
        let mut count = 1;
        let mut power = 1;

        while let Some(ch0) = s.next() {
            if let Some(&ch1) = s.peek() {
                if ch1 == ch0 {
                    count += 1;
                } else {
                    power = power.max(count);
                    count = 1;
                }
            }
        }

        power.max(count)
    }
}
