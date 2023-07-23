impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ret = vec![];

        for ch0 in s.bytes() {
            if ch0 == b')' {
                let mut stack = vec![];

                while let Some(ch1) = ret.pop() {
                    if ch1 == b'(' {
                        break;
                    }
                    stack.push(ch1);
                }

                ret.append(&mut stack);
            } else {
                ret.push(ch0);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
