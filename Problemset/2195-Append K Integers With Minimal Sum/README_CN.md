# 2195. 向数组中追加 K 个整数
给你一个整数数组 `nums` 和一个整数 `k` 。请你向 `nums` 中追加 `k` 个 **未** 出现在 `nums` 中的、**互不相同** 的 **正** 整数，并使结果数组的元素和 **最小** 。

返回追加到 `nums` 中的 `k` 个整数之和。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,4,25,10,25], k = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 在该解法中，向数组中追加的两个互不相同且未出现的正整数是 2 和 3 。
nums 最终元素和为 1 + 4 + 25 + 10 + 25 + 2 + 3 = 70 ，这是所有情况中的最小值。
所以追加到数组中的两个整数之和是 2 + 3 = 5 ，所以返回 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,6], k = 6
<strong>输出:</strong> 25
<strong>解释:</strong> 在该解法中，向数组中追加的两个互不相同且未出现的正整数是 1 、2 、3 、4 、7 和 8 。
nums 最终元素和为 5 + 6 + 1 + 2 + 3 + 4 + 7 + 8 = 36 ，这是所有情况中的最小值。
所以追加到数组中的两个整数之和是 1 + 2 + 3 + 4 + 7 + 8 = 25 ，所以返回 25 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        let mut k = k;
        let mut ret = 0;

        nums.push(0);
        nums.push(i32::MAX);
        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;
            } else if nums[i] - nums[i - 1] - 1 < k {
                k -= nums[i] - nums[i - 1] - 1;
                ret +=
                    (nums[i - 1] as i64 + nums[i] as i64) * (nums[i] - nums[i - 1] - 1) as i64 / 2;
            } else {
                ret += (nums[i - 1] as i64 * 2 + 1 + k as i64) * k as i64 / 2;
                break;
            }
        }

        ret
    }
}
```
