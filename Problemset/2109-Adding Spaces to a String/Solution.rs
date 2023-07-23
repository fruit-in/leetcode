impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = vec![];

        for i in spaces {
            for j in prev..i as usize {
                ret.push(s[j]);
            }
            ret.push(b' ');
            prev = i as usize;
        }

        for j in prev..s.len() {
            ret.push(s[j]);
        }

        String::from_utf8(ret).unwrap()
    }
}
