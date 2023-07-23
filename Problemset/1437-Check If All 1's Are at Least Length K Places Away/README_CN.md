# 1437. 是否所有 1 都至少相隔 k 个元素
给你一个由若干 `0` 和 `1` 组成的数组 `nums` 以及整数 `k`。如果所有 `1` 都至少相隔 `k` 个元素，则返回 `True` ；否则，返回 `False` 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/05/03/sample_1_1791.png)
<pre>
<strong>输入:</strong> nums = [1,0,0,0,1,0,0,1], k = 2
<strong>输出:</strong> true
<strong>解释:</strong> 每个 1 都至少相隔 2 个元素。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/05/03/sample_2_1791.png)
<pre>
<strong>输入:</strong> nums = [1,0,0,1,0,1], k = 2
<strong>输出:</strong> false
<strong>解释:</strong> 第二个 1 和第三个 1 之间只隔了 1 个元素。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], k = 0
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [0,1,0,1], k = 1
<strong>输出:</strong> true
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `0 <= k <= nums.length`
* `nums[i]` 的值为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_1 = -k - 1;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                if i as i32 - prev_1 <= k {
                    return false;
                }
                prev_1 = i as i32;
            }
        }

        true
    }
}
```
