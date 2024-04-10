# 1498. 满足条件的子序列数目
给你一个整数数组 `nums` 和一个整数 `target` 。

请你统计并返回 `nums` 中能满足其最小元素与最大元素的 **和** 小于或等于 `target` 的 **非空** 子序列的数目。

由于答案可能很大，请将结果对 <code>10<sup>9</sup> + 7</code> 取余后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,5,6,7], target = 9
<strong>输出:</strong> 4
<strong>解释:</strong> 有 4 个子序列满足该条件。
[3] -> 最小元素 + 最大元素 <= target (3 + 3 <= 9)
[3,5] -> (3 + 5 <= 9)
[3,5,6] -> (3 + 6 <= 9)
[3,6] -> (3 + 6 <= 9)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,3,6,8], target = 10
<strong>输出:</strong> 6
<strong>解释:</strong> 有 6 个子序列满足该条件。（nums 中可以有重复数字）
[3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,3,3,4,6,7], target = 12
<strong>输出:</strong> 61
<strong>解释:</strong> 共有 63 个非空子序列，其中 2 个不满足条件（[6,7], [7]）
有效序列总数为（63 - 2 = 61）
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* <code>1 <= target <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut i = nums.len() - 1;
        let mut pow2 = vec![1];
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j > i || nums[j] * 2 > target {
                break;
            }

            while nums[i] + nums[j] > target {
                i -= 1;
            }

            while i - j >= pow2.len() {
                pow2.push(pow2.last().unwrap() * 2 % 1_000_000_007);
            }

            ret = (ret + pow2[i - j]) % 1_000_000_007;
        }

        ret
    }
}
```
