# 2009. 使数组连续的最少操作数
给你一个整数数组 `nums` 。每一次操作中，你可以将 `nums` 中 **任意** 一个元素替换成 **任意** 整数。

如果 `nums` 满足以下条件，那么它是 **连续的** ：
* `nums` 中所有元素都是 **互不相同** 的。
* `nums` 中 **最大** 元素与 **最小** 元素的差等于 `nums.length - 1` 。

比方说，`nums = [4, 2, 5, 3]` 是 **连续的** ，但是 `nums = [1, 2, 3, 5, 6]` **不是连续的** 。

请你返回使 `nums` **连续** 的 **最少** 操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,2,5,3]
<strong>输出:</strong> 0
<strong>解释:</strong> nums 已经是连续的了。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,5,6]
<strong>输出:</strong> 1
<strong>解释:</strong> 一个可能的解是将最后一个元素变为 4 。
结果数组为 [1,2,3,5,4] ，是连续数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,10,100,1000]
<strong>输出:</strong> 3
<strong>解释:</strong> 一个可能的解是：
- 将第二个元素变为 2 。
- 将第三个元素变为 3 。
- 将第四个元素变为 4 。
结果数组为 [1,2,3,4] ，是连续数组。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut i = 0;
        let mut count = 0;
        let mut ret = n;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j == 0 || nums[j] != nums[j - 1] {
                count += 1;
            }
            while nums[j] - nums[i] > n - 1 {
                if i == 0 || nums[i] != nums[i - 1] {
                    count -= 1;
                }
                i += 1;
            }

            ret = ret.min(n - count);
        }

        ret
    }
}
```
