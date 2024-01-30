# 1673. 找出最具竞争力的子序列
给你一个整数数组 `nums` 和一个正整数 `k` ，返回长度为 `k` 且最具 **竞争力** 的 `nums` 子序列。

数组的子序列是从数组中删除一些元素（可能不删除元素）得到的序列。

在子序列 `a` 和子序列 `b` 第一个不相同的位置上，如果 `a` 中的数字小于 `b` 中对应的数字，那么我们称子序列 `a` 比子序列 `b`（相同长度下）更具 **竞争力** 。 例如，`[1,3,4]` 比 `[1,3,5]` 更具竞争力，在第一个不相同的位置，也就是最后一个位置上， `4` 小于 `5` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,5,2,6], k = 2
<strong>输出:</strong> [2,6]
<strong>解释:</strong> 在所有可能的子序列集合 {[3,5], [3,2], [3,6], [5,2], [5,6], [2,6]} 中，[2,6] 最具竞争力。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,4,3,3,5,4,9,6], k = 4
<strong>输出:</strong> [2,3,3,4]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ret = vec![];

        for i in 0..nums.len() {
            while let Some(&num) = ret.last() {
                if num > nums[i] && k - ret.len() < nums.len() - i {
                    ret.pop();
                } else {
                    break;
                }
            }

            if ret.len() < k {
                ret.push(nums[i]);
            }
        }

        ret
    }
}
```
