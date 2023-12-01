impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut count = s.chars().filter(|&c| c == 'a').count() as i32;
        let mut ret = count;

        for c in s.chars() {
            count += (c == 'b') as i32 - (c == 'a') as i32;
            ret = ret.min(count);
        }

        ret
    }
}
