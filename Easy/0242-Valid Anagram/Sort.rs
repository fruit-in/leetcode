impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();
        s_vec.sort_unstable();
        t_vec.sort_unstable();
        s_vec == t_vec
    }
}
