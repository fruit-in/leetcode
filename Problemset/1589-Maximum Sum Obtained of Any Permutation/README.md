# 1589. Maximum Sum Obtained of Any Permutation
We have an array of integers, `nums`, and an array of `requests` where <code>requests[i] = [start<sub>i</sub>, end<sub>i</sub>]</code>. The <code>i<sup>th</sup></code> request asks for the sum of <code>nums[start<sub>i</sub>] + nums[start<sub>i</sub> + 1] + ... + nums[end<sub>i</sub> - 1] + nums[end<sub>i</sub>]</code>. Both <code>start<sub>i</sub></code> and <code>end<sub>i</sub></code> are *0-indexed*.

Return *the maximum total sum of all requests **among all permutations** of* `nums`.

Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5], requests = [[1,3],[0,1]]
<strong>Output:</strong> 19
<strong>Explanation:</strong> One permutation of nums is [2,1,3,4,5] with the following result:
requests[0] -> nums[1] + nums[2] + nums[3] = 1 + 3 + 4 = 8
requests[1] -> nums[0] + nums[1] = 2 + 1 = 3
Total sum: 8 + 3 = 11.
A permutation with a higher total sum is [3,5,4,2,1] with the following result:
requests[0] -> nums[1] + nums[2] + nums[3] = 5 + 4 + 2 = 11
requests[1] -> nums[0] + nums[1] = 3 + 5  = 8
Total sum: 11 + 8 = 19, which is the best that you can do.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6], requests = [[0,1]]
<strong>Output:</strong> 11
<strong>Explanation:</strong> A permutation with the max total sum is [6,5,4,3,2,1] with request sums [11].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,10], requests = [[0,2],[1,3],[1,1]]
<strong>Output:</strong> 47
<strong>Explanation:</strong> A permutation with the max total sum is [4,10,5,3,2,1] with request sums [19,18,10].
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= requests.length <= 10<sup>5</sup></code>
* `requests[i].length == 2`
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < n</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let mut suffix_sum = vec![0; nums.len()];

        for request in &requests {
            if request[0] > 0 {
                suffix_sum[request[0] as usize - 1] -= 1;
            }
            suffix_sum[request[1] as usize] += 1;
        }

        for i in (0..suffix_sum.len() - 1).rev() {
            suffix_sum[i] += suffix_sum[i + 1];
        }

        nums.sort_unstable();
        suffix_sum.sort_unstable();

        (nums
            .iter()
            .zip(suffix_sum.iter())
            .map(|(x, y)| *x as i64 * *y as i64)
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}
```
