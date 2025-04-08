# 862. 和至少为 K 的最短子数组
给你一个整数数组 `nums` 和一个整数 `k` ，找出 `nums` 中和至少为 `k` 的 **最短非空子数组** ，并返回该子数组的长度。如果不存在这样的 **子数组** ，返回 `-1` 。

**子数组** 是数组中 **连续** 的一部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1], k = 1
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2], k = 4
<strong>输出:</strong> -1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,-1,2], k = 3
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut prefix_sum = 0;
        let mut stack = vec![(0, -1)];
        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;

            while stack.last().unwrap_or(&(i64::MIN, 0)).0 >= prefix_sum {
                stack.pop();
            }
            stack.push((prefix_sum, i as i32));

            let j = stack
                .binary_search(&(prefix_sum - k, i as i32))
                .unwrap_err();
            if j > 0 {
                ret = ret.min(i as i32 - stack[j - 1].1);
            }
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }
}
```
