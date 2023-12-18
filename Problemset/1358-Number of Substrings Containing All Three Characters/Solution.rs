impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 3];
        let mut i = 0;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'a') as usize] += 1;

            while count[0] * count[1] * count[2] > 0 {
                count[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }

            ret += i;
        }

        ret as i32
    }
}
