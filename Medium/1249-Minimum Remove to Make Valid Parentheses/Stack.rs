impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut lp = vec![];
        let mut ret = vec![];

        for ch in s.bytes() {
            if ch != b')' || lp.len() > 0 {
                ret.push(ch);
            }
            if ch == b'(' {
                lp.push(ret.len() - 1);
            } else if ch == b')' {
                lp.pop();
            }
        }

        while let Some(i) = lp.pop() {
            ret.remove(i);
        }

        String::from_utf8(ret).unwrap()
    }
}
