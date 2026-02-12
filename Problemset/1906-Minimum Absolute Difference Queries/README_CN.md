# 1906. 查询差绝对值的最小值
一个数组 `a` 的 **差绝对值的最小值** 定义为：`0 <= i < j < a.length` 且 `a[i] != a[j]` 的 `|a[i] - a[j]|` 的 **最小值**。如果 `a` 中所有元素都 **相同** ，那么差绝对值的最小值为 `-1` 。

* 比方说，数组 `[5,2,3,7,2]` 差绝对值的最小值是 `|2 - 3| = 1` 。注意答案不为 `0` ，因为 `a[i]` 和 `a[j]` 必须不相等。

给你一个整数数组 `nums` 和查询数组 `queries` ，其中 <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> 。对于每个查询 `i` ，计算 **子数组** <code>nums[l<sub>i</sub>...r<sub>i</sub>]</code> 中 **差绝对值的最小值** ，子数组 <code>nums[l<sub>i</sub>...r<sub>i</sub>]</code> 包含 `nums` 数组（下标从 **0** 开始）中下标在 <code>l<sub>i</sub></code> 和 <code>r<sub>i</sub></code> 之间的所有元素（包含 <code>l<sub>i</sub></code> 和 <code>r<sub>i</sub></code> 在内）。

请你返回 `ans` **数组**，其中 `ans[i]` 是第 `i` 个查询的答案。

**子数组** 是一个数组中连续的一段元素。

`|x|` 的值定义为：
* 如果 `x >= 0` ，那么值为 `x` 。
* 如果 `x < 0` ，那么值为 `-x` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,4,8], queries = [[0,1],[1,2],[2,3],[0,3]]
<strong>输出:</strong> [2,1,4,1]
<strong>解释:</strong> 查询结果如下：
- queries[0] = [0,1]：子数组是 [1,3] ，差绝对值的最小值为 |1-3| = 2 。
- queries[1] = [1,2]：子数组是 [3,4] ，差绝对值的最小值为 |3-4| = 1 。
- queries[2] = [2,3]：子数组是 [4,8] ，差绝对值的最小值为 |4-8| = 4 。
- queries[3] = [0,3]：子数组是 [1,3,4,8] ，差的绝对值的最小值为 |3-4| = 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,5,2,2,7,10], queries = [[2,3],[0,2],[0,5],[3,5]]
<strong>输出:</strong> [-1,1,1,3]
<strong>解释:</strong> 查询结果如下：
- queries[0] = [2,3]：子数组是 [2,2] ，差绝对值的最小值为 -1 ，因为所有元素相等。
- queries[1] = [0,2]：子数组是 [4,5,2] ，差绝对值的最小值为 |4-5| = 1 。
- queries[2] = [0,5]：子数组是 [4,5,2,2,7,10] ，差绝对值的最小值为 |4-5| = 1 。
- queries[3] = [3,5]：子数组是 [2,7,10] ，差绝对值的最小值为 |7-10| = 3 。
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 100`
* <code>1 <= queries.length <= 2 * 10<sup>4</sup></code>
* <code>0 <= l<sub>i</sub> < r<sub>i</sub> < nums.length</code>

## 题解 (Rust)

### 1. 题解
```Rust
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
```
