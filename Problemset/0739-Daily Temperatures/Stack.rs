impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; t.len()];

        for i in (0..t.len()).rev() {
            while stack.last().unwrap_or(&(101, 0)).0 <= t[i] {
                stack.pop();
            }
            ret[i] = (stack.last().unwrap_or(&(0, i)).1 - i) as i32;
            stack.push((t[i], i));
        }

        ret
    }
}
