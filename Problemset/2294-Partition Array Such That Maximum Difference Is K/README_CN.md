# 2294. 划分数组使最大差为 K
给你一个整数数组 `nums` 和一个整数 `k` 。你可以将 `nums` 划分成一个或多个 **子序列** ，使 `nums` 中的每个元素都 **恰好** 出现在一个子序列中。

在满足每个子序列中最大值和最小值之间的差值最多为 `k` 的前提下，返回需要划分的 **最少** 子序列数目。

**子序列** 本质是一个序列，可以通过删除另一个序列中的某些元素（或者不删除）但不改变剩下元素的顺序得到。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,6,1,2,5], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
可以将 nums 划分为两个子序列 [3,1,2] 和 [6,5] 。
第一个子序列中最大值和最小值的差值是 3 - 1 = 2 。
第二个子序列中最大值和最小值的差值是 6 - 5 = 1 。
由于创建了两个子序列，返回 2 。可以证明需要划分的最少子序列数目就是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3], k = 1
<strong>输出:</strong> 2
<strong>解释:</strong>
可以将 nums 划分为两个子序列 [1,2] 和 [3] 。
第一个子序列中最大值和最小值的差值是 2 - 1 = 1 。
第二个子序列中最大值和最小值的差值是 3 - 3 = 0 。
由于创建了两个子序列，返回 2 。注意，另一种最优解法是将 nums 划分成子序列 [1] 和 [2,3] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,2,4,5], k = 0
<strong>输出:</strong> 3
<strong>解释:</strong>
可以将 nums 划分为三个子序列 [2,2]、[4] 和 [5] 。
第一个子序列中最大值和最小值的差值是 2 - 2 = 0 。
第二个子序列中最大值和最小值的差值是 4 - 4 = 0 。
第三个子序列中最大值和最小值的差值是 5 - 5 = 0 。
由于创建了三个子序列，返回 3 。可以证明需要划分的最少子序列数目就是 3 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>
* <code>0 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut x = -k - 1;
        let mut ret = 0;
        nums.sort_unstable();

        for num in nums {
            if num - x > k {
                x = num;
                ret += 1;
            }
        }

        ret
    }
}
```
