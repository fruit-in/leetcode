# 2155. 分组得分最高的所有下标
给你一个下标从 **0** 开始的二进制数组 `nums` ，数组长度为 `n` 。`nums` 可以按下标 `i`（ `0 <= i <= n` ）拆分成两个数组（可能为空）：<code>nums<sub>left</sub></code> 和 <code>nums<sub>right</sub></code> 。

* <code>nums<sub>left</sub></code> 包含 `nums` 中从下标 `0` 到 `i - 1` 的所有元素（**包括** `0` **和** `i - 1` ），而 <code>nums<sub>right</sub></code> 包含 `nums` 中从下标 `i` 到 `n - 1` 的所有元素（**包括** `i` **和** `n - 1` ）。
* 如果 `i == 0` ，<code>nums<sub>left</sub></code> 为 **空** ，而 <code>nums<sub>right</sub></code> 将包含 `nums` 中的所有元素。
* 如果 `i == n` ，<code>nums<sub>left</sub></code> 将包含 `nums` 中的所有元素，而 <code>nums<sub>right</sub></code> 为 **空** 。

下标 `i` 的 **分组得分** 为 <code>nums<sub>left</sub></code> 中 `0` 的个数和 <code>nums<sub>right</sub></code> 中 `1` 的个数之 **和** 。

返回 **分组得分 最高** 的 **所有不同下标** 。你可以按 **任意顺序** 返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,0,1,0]
<strong>输出:</strong> [2,4]
<strong>解释:</strong> 按下标分组
- 0 ：numsleft 为 [] 。numsright 为 [0,0,1,0] 。得分为 0 + 1 = 1 。
- 1 ：numsleft 为 [0] 。numsright 为 [0,1,0] 。得分为 1 + 1 = 2 。
- 2 ：numsleft 为 [0,0] 。numsright 为 [1,0] 。得分为 2 + 1 = 3 。
- 3 ：numsleft 为 [0,0,1] 。numsright 为 [0] 。得分为 2 + 0 = 2 。
- 4 ：numsleft 为 [0,0,1,0] 。numsright 为 [] 。得分为 3 + 0 = 3 。
下标 2 和 4 都可以得到最高的分组得分 3 。
注意，答案 [4,2] 也被视为正确答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,0]
<strong>输出:</strong> [3]
<strong>Explanation:</strong> 按下标分组
- 0 ：numsleft 为 [] 。numsright 为 [0,0,0] 。得分为 0 + 0 = 0 。
- 1 ：numsleft 为 [0] 。numsright 为 [0,0] 。得分为 1 + 0 = 1 。
- 2 ：numsleft 为 [0,0] 。numsright 为 [0] 。得分为 2 + 0 = 2 。
- 3 ：numsleft 为 [0,0,0] 。numsright 为 [] 。得分为 3 + 0 = 3 。
只有下标 3 可以得到最高的分组得分 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1]
<strong>输出:</strong> [0]
<strong>Explanation:</strong> 按下标分组
- 0 ：numsleft 为 [] 。numsright 为 [1,1] 。得分为 0 + 2 = 2 。
- 1 ：numsleft 为 [1] 。numsright 为 [1] 。得分为 0 + 1 = 1 。
- 2 ：numsleft 为 [1,1] 。numsright 为 [] 。得分为 0 + 0 = 0 。
只有下标 0 可以得到最高的分组得分 2 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `nums[i]` 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut left0 = 0;
        let mut right1 = nums.iter().filter(|&&x| x == 1).count();
        let mut max_score = right1;
        let mut ret = vec![0];

        for i in 0..nums.len() {
            match nums[i] {
                0 => left0 += 1,
                _ => right1 -= 1,
            }

            if left0 + right1 > max_score {
                max_score = left0 + right1;
                ret = vec![i as i32 + 1];
            } else if left0 + right1 == max_score {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```
