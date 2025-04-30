# 1703. Minimum Adjacent Swaps for K Consecutive Ones
You are given an integer array, `nums`, and an integer `k`. `nums` comprises of only `0`'s and `1`'s. In one move, you can choose two **adjacent** indices and swap their values.

Return *the **minimum** number of moves required so that* `nums` *has* `k` ***consecutive*** `1`*'s*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,0,0,1,0,1], k = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> In 1 move, nums could be [1,0,0,0,1,1] and have 2 consecutive 1's.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,0,0,0,0,0,1,1], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> In 5 moves, the leftmost 1 can be shifted right until nums = [0,0,0,0,0,1,1,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,0,1], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums already has 2 consecutive 1's.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` is `0` or `1`.
* `1 <= k <= sum(nums)`

## Solutions (Rust)

### 1. Solution
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
