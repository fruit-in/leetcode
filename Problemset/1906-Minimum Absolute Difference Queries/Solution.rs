impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut prefix_count = vec![vec![0; max_num + 1]];
        let mut ans = vec![-1; queries.len()];

        for i in 0..nums.len() {
            let mut count = prefix_count[i].clone();
            count[nums[i] as usize] += 1;
            prefix_count.push(count);
        }

        for i in 0..queries.len() {
            let (l, r) = (queries[i][0] as usize, queries[i][1] as usize);
            let mut prev = 0;

            for j in 1..=max_num {
                if prefix_count[r + 1][j] - prefix_count[l][j] > 0 {
                    if prev > 0 && (ans[i] == -1 || j as i32 - prev < ans[i]) {
                        ans[i] = j as i32 - prev;
                    }
                    prev = j as i32;
                }
            }
        }

        ans
    }
}
