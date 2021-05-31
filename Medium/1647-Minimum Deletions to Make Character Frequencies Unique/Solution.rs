impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counter = vec![0; 26];
        let mut ret = 0;

        for c in s.bytes() {
            counter[(c - b'a') as usize] += 1;
        }
        counter.sort_unstable();

        for i in (1..26).rev() {
            if counter[i] <= counter[i - 1] {
                ret += (counter[i - 1] - counter[i] + 1).min(counter[i - 1]);
                counter[i - 1] = (counter[i] - 1).max(0);
            }
        }

        ret
    }
}
