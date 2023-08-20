impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; heights.len()];

        for i in (0..heights.len()).rev() {
            while let Some(&height) = stack.last() {
                ret[i] += 1;
                if heights[i] > height {
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(heights[i]);
        }

        ret
    }
}
