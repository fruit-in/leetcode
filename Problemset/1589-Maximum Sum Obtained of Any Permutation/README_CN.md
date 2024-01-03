# 1589. 所有排列中的最大和
有一个整数数组 `nums` ，和一个查询数组 `requests` ，其中 <code>requests[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 。第 `i` 个查询求 <code>nums[start<sub>i</sub>] + nums[start<sub>i</sub> + 1] + ... + nums[end<sub>i</sub> - 1] + nums[end<sub>i</sub>]</code> 的结果 ，<code>start<sub>i</sub></code> 和 <code>end<sub>i</sub></code> 数组索引都是 **从 0 开始** 的。

你可以任意排列 `nums` 中的数字，请你返回所有查询结果之和的最大值。

由于答案可能会很大，请你将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5], requests = [[1,3],[0,1]]
<strong>输出:</strong> 19
<strong>解释:</strong> 一个可行的 nums 排列为 [2,1,3,4,5]，并有如下结果：
requests[0] -> nums[1] + nums[2] + nums[3] = 1 + 3 + 4 = 8
requests[1] -> nums[0] + nums[1] = 2 + 1 = 3
总和为：8 + 3 = 11。
一个总和更大的排列为 [3,5,4,2,1]，并有如下结果：
requests[0] -> nums[1] + nums[2] + nums[3] = 5 + 4 + 2 = 11
requests[1] -> nums[0] + nums[1] = 3 + 5  = 8
总和为： 11 + 8 = 19，这个方案是所有排列中查询之和最大的结果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6], requests = [[0,1]]
<strong>输出:</strong> 11
<strong>解释:</strong> 一个总和最大的排列为 [6,5,4,3,2,1] ，查询和为 [11]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,10], requests = [[0,2],[1,3],[1,1]]
<strong>输出:</strong> 47
<strong>解释:</strong> 一个和最大的排列为 [4,10,5,3,2,1] ，查询结果分别为 [19,18,10]。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= requests.length <= 10<sup>5</sup></code>
* `requests[i].length == 2`
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < n</code>

## 题解 (Rust)

### 1. 题解
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
