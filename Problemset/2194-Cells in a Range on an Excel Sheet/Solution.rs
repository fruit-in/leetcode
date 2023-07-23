impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut ret = vec![];

        for col in s[0]..=s[3] {
            for row in s[1]..=s[4] {
                ret.push(String::from_utf8(vec![col, row]).unwrap());
            }
        }

        ret
    }
}
