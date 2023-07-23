impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ret = vec![];

        for b in s.bytes() {
            if b == b'*' {
                ret.pop();
            } else {
                ret.push(b);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
