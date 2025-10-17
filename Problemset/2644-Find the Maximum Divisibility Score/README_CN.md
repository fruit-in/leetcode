# 2644. 找出可整除性得分最大的整数
给你两个整数数组 `nums` 和 `divisors` 。

`divisors[i]` 的 **可整除性得分** 等于满足 `nums[j]` 能被 `divisors[i]` 整除的下标 `j` 的数量。

返回 **可整除性得分** 最大的整数 `divisors[i]` 。如果有多个整数具有最大得分，则返回数值最小的一个。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,9,15,50], divisors = [5,3,7,2]
<strong>输出:</strong> 2
<strong>解释:</strong>
divisors[0] 的可整除性分数为 2 因为 nums[2] 和 nums[3] 能被 5 整除。
divisors[1] 的可整除性分数为 2 因为 nums[1] 和 nums[2] 能被 3 整除。
divisors[2] 的可整除性分数为 0 因为 nums 中没有数字能被 7 整除。
divisors[3] 的可整除性分数为 2 因为 nums[0] 和 nums[3] 能够被 2 整除。
因为 divisors[0] 、divisor[1] 和 divisors[3] 有相同的可整除性分数，我们返回更小的那个 divisors[3]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,7,9,3,9], divisors = [5,2,3]
<strong>输出:</strong> 3
<strong>解释:</strong>
divisors[0] 的可整除性分数为 0 因为 nums 中没有数字能被 5 整除。
divisors[1] 的可整除性分数为 1 因为只有 nums[0] 能被 2 整除。
divisors[2] 的可整除性分数为 3 因为 nums[2] ，nums[3] 和 nums[4] 能被 3 整除。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [20,14,21,10], divisors = [10,16,20]
<strong>输出:</strong> 10
<strong>解释:</strong>
divisors[0] 的可整除性分数为 2 因为 nums[0] 和 nums[3] 能被 10 整除。
divisors[1] 的可整除性分数为 0 因为 nums 中没有数字能被 16 整除。
divisors[2] 的可整除性分数为 1 因为 nums[0] 能被 20 整除。
因为 divisors[0] 的可整除性分数最大，我们返回 divisors[0]。
</pre>

#### 提示:
* `1 <= nums.length, divisors.length <= 1000`
* <code>1 <= nums[i], divisors[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut ret = i32::MAX;

        for i in 0..divisors.len() {
            let mut score = 0;

            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    score += 1;
                }
            }

            if score > max_score {
                max_score = score;
                ret = divisors[i];
            } else if score == max_score && divisors[i] < ret {
                ret = divisors[i];
            }
        }

        ret
    }
}
```
