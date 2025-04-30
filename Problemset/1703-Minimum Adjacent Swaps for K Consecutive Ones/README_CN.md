# 1703. 得到连续 K 个 1 的最少相邻交换次数
给你一个整数数组 `nums` 和一个整数 `k` 。 `nums` 仅包含 `0` 和 `1` 。每一次移动，你可以选择 **相邻** 两个数字并将它们交换。

请你返回使 `nums` 中包含 `k` 个 **连续** `1` 的 **最少** 交换次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,0,0,1,0,1], k = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 在第一次操作时，nums 可以变成 [1,0,0,0,1,1] 得到连续两个 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,0,0,0,0,0,1,1], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 通过 5 次操作，最左边的 1 可以移到右边直到 nums 变为 [0,0,0,0,0,1,1,1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,0,1], k = 2
<strong>输出:</strong> 0
<strong>解释:</strong> nums 已经有连续 2 个 1 了。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` 要么是 `0` ，要么是 `1` 。
* `1 <= k <= sum(nums)`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut moves = k / 2 * (k % 2 - 1);
        let mut l = 0;
        let mut m = 0;
        let mut r = 0;
        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                count += 1;
                if count <= (k - 1) / 2 {
                    moves -= count * 2;
                }
                if count == 1 {
                    l = i;
                }
                if count == (k + 1) / 2 {
                    m = i;
                }
                if count == k {
                    r = i;
                    break;
                }
            }
        }

        for i in l..m {
            if nums[i] == 1 {
                moves += (m - i) as i32;
            }
        }
        for i in m + 1..=r {
            if nums[i] == 1 {
                moves += (i - m) as i32;
            }
        }
        ret = ret.min(moves);

        loop {
            moves += l as i32;
            moves -= m as i32 * (k % 2);
            l += 1;
            m += 1;
            r += 1;
            while r < nums.len() && nums[r] == 0 {
                r += 1;
            }
            if r == nums.len() {
                break;
            }
            while nums[m] == 0 {
                m += 1;
            }
            while nums[l] == 0 {
                l += 1;
            }
            moves += r as i32;
            moves -= m as i32 * (2 - k % 2);
            ret = ret.min(moves);
        }

        ret
    }
}
```
