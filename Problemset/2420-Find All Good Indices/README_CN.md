# 2420. 找到所有好下标
给你一个大小为 `n` 下标从 **0** 开始的整数数组 `nums` 和一个正整数 `k` 。

对于 `k <= i < n - k` 之间的一个下标 `i` ，如果它满足以下条件，我们就称它为一个 **好** 下标：

* 下标 `i` **之前** 的 `k` 个元素是 **非递增的** 。
* 下标 `i` **之后** 的 `k` 个元素是 **非递减的** 。

按 **升序** 返回所有好下标。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,1,1,3,4,1], k = 2
<strong>输出:</strong> [2,3]
<strong>解释:</strong> 数组中有两个好下标：
- 下标 2 。子数组 [2,1] 是非递增的，子数组 [1,3] 是非递减的。
- 下标 3 。子数组 [1,1] 是非递增的，子数组 [3,4] 是非递减的。
注意，下标 4 不是好下标，因为 [4,1] 不是非递减的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,1,1,2], k = 2
<strong>输出:</strong> []
<strong>解释:</strong> 数组中没有好下标。
</pre>

#### 提示:
* `n == nums.length`
* <code>3 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= n / 2`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut count = (1, 1);

        for i in 1..n {
            prefix[i] = count.0;
            suffix[n - 1 - i] = count.1;

            if nums[i] <= nums[i - 1] {
                count.0 += 1;
            } else {
                count.0 = 1;
            }
            if nums[n - 1 - i] <= nums[n - i] {
                count.1 += 1;
            } else {
                count.1 = 1;
            }
        }

        (k..n - k)
            .filter(|&i| prefix[i].min(suffix[i]) >= k)
            .map(|i| i as i32)
            .collect()
    }
}
```
