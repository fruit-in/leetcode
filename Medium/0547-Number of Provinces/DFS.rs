impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let mut seen = vec![false; m.len()];
        let mut stack = vec![];
        let mut ret = 0;

        for i in 0..m.len() {
            if !seen[i] {
                ret += 1;
                stack.push(i);

                while let Some(c) = stack.pop() {
                    seen[c] = true;
                    for j in i + 1..m.len() {
                        if !seen[j] && m[c][j] == 1 {
                            stack.push(j);
                        }
                    }
                }
            }
        }

        ret
    }
}
