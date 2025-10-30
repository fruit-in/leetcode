impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut count0 = 0;
        let mut wait = 0;

        for c in s.trim_start_matches('1').trim_end_matches('0').chars() {
            if c == '0' {
                count0 += 1;
                wait -= 1;
            } else {
                wait = (wait + 1).max(0);
            }
        }

        count0 + wait
    }
}
