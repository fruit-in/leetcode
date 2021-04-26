impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut count_p = [0; 26];
        let mut count_s = [0; 26];
        let mut ret = vec![];

        for i in 0..p.len() {
            count_p[(p[i] - b'a') as usize] += 1;
            count_s[(s[i] - b'a') as usize] += 1;
        }

        for i in 0..=s.len() - p.len() {
            if count_p == count_s {
                ret.push(i as i32);
            }
            if i + p.len() < s.len() {
                count_s[(s[i] - b'a') as usize] -= 1;
                count_s[(s[i + p.len()] - b'a') as usize] += 1;
            }
        }

        ret
    }
}
