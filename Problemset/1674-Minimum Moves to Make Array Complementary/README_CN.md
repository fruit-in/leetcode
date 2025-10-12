# 1674. 使数组互补的最少操作次数
给你一个长度为 **偶数** `n` 的整数数组 `nums` 和一个整数 `limit` 。每一次操作，你可以将 `nums` 中的任何整数替换为 `1` 到 `limit` 之间的另一个整数。

如果对于所有下标 `i`（**下标从 `0` 开始**），`nums[i] + nums[n - 1 - i]` 都等于同一个数，则数组 `nums` 是 **互补的** 。例如，数组 `[1,2,3,4]` 是互补的，因为对于所有下标 `i` ，`nums[i] + nums[n - 1 - i] = 5` 。

返回使数组 **互补** 的 **最少** 操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,4,3], limit = 4
<strong>输出:</strong> 1
<strong>解释:</strong> 经过 1 次操作，你可以将数组 nums 变成 [1,2,2,3]（加粗元素是变更的数字）：
nums[0] + nums[3] = 1 + 3 = 4.
nums[1] + nums[2] = 2 + 2 = 4.
nums[2] + nums[1] = 2 + 2 = 4.
nums[3] + nums[0] = 3 + 1 = 4.
对于每个 i ，nums[i] + nums[n-1-i] = 4 ，所以 nums 是互补的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,2,1], limit = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 经过 2 次操作，你可以将数组 nums 变成 [2,2,2,2] 。你不能将任何数字变更为 3 ，因为 3 > limit 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,1,2], limit = 2
<strong>输出:</strong> 0
<strong>解释:</strong> nums 已经是互补的。
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= limit <= 10<sup>5</sup></code>
* `n` 是偶数。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let n = nums.len();
        let mut arr = vec![0; limit * 2 + 2];
        let mut prefix_sum = 0;
        let mut ret = i32::MAX;

        for i in 0..n / 2 {
            let x = nums[i] as usize;
            let y = nums[n - 1 - i] as usize;

            arr[2] += 2;
            arr[x.min(y) + 1] -= 1;
            arr[x.max(y) + limit + 1] += 1;
            arr[x + y] -= 1;
            arr[x + y + 1] += 1;
        }

        for i in 2..=limit * 2 {
            prefix_sum += arr[i];
            ret = ret.min(prefix_sum);
        }

        ret
    }
}
```
