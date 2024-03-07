impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut widths = vec![1; heights.len()];
        let mut ret = 0;

        for i in 0..heights.len() {
            while stack.last().unwrap_or(&(0, -1)).1 >= heights[i] {
                stack.pop();
            }

            widths[i] = i as i32 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i32, heights[i]));
        }

        stack.clear();

        for i in (0..heights.len()).rev() {
            while stack.last().unwrap_or(&(0, -1)).1 >= heights[i] {
                stack.pop();
            }

            widths[i] += stack.last().unwrap_or(&(heights.len() as i32, 0)).0 - 1 - i as i32;
            stack.push((i as i32, heights[i]));
            ret = ret.max(widths[i] * heights[i]);
        }

        ret
    }
}
