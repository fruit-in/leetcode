impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if x < y {
            return Self::maximum_gain(
                s.replace("a", "$").replace("b", "a").replace("$", "b"),
                y,
                x,
            );
        }

        let mut stack1 = vec![];
        let mut ret = 0;

        for c in s.chars().chain(std::iter::once('c')) {
            if c == 'b' && stack1.last() == Some(&'a') {
                stack1.pop();
                ret += x;
            } else if c == 'a' || c == 'b' {
                stack1.push(c);
            } else {
                let mut stack2 = vec![];

                for c in stack1.drain(..) {
                    if c == 'a' && stack2.last() == Some(&'b') {
                        stack2.pop();
                        ret += y;
                    } else {
                        stack2.push(c);
                    }
                }
            }
        }

        ret
    }
}
