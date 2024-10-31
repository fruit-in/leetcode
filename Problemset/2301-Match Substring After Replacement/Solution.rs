impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let mut bitmappings = [0_i128; 128];

        for i in 0..mappings.len() {
            let (old, new) = (mappings[i][0] as usize, mappings[i][1] as usize);
            bitmappings[old] |= 1 << new;
        }

        for i in 0..=s.len() - sub.len() {
            let mut flag = true;

            for j in 0..sub.len() {
                if sub[j] != s[i + j] && bitmappings[sub[j] as usize] & (1 << s[i + j]) == 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                return true;
            }
        }

        false
    }
}
