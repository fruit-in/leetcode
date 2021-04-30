impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];

        for i in 0..s1.len() {
            count1[(s1[i] - b'a') as usize] += 1;
            count2[(s2[i] - b'a') as usize] += 1;
        }

        for i in 0..=s2.len() - s1.len() {
            if count1 == count2 {
                return true;
            }
            if i + s1.len() < s2.len() {
                count2[(s2[i] - b'a') as usize] -= 1;
                count2[(s2[i + s1.len()] - b'a') as usize] += 1;
            }
        }

        false
    }
}
