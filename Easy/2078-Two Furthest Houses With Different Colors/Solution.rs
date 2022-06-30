impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut ret = n - 1;

        while colors[0] == colors[ret] && colors[n - 1] == colors[n - 1 - ret] {
            ret -= 1;
        }

        ret as i32
    }
}
