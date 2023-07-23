impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut count = [0; 26];

        s.bytes().for_each(|c| count[(c - b'a') as usize] += 1);

        for i in 1..26 {
            if count[i] == 0 {
                count[i] = count[i - 1];
            } else if count[i - 1] != 0 && count[i] != count[i - 1] {
                return false;
            }
        }

        true
    }
}
