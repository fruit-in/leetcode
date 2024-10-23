# 1775. 通过最少操作次数使数组的和相等
给你两个长度可能不等的整数数组 `nums1` 和 `nums2` 。两个数组中的所有值都在 `1` 到 `6` 之间（包含 `1` 和 `6`）。

每次操作中，你可以选择 **任意** 数组中的任意一个整数，将它变成 `1` 到 `6` 之间 **任意** 的值（包含 `1` 和 `6`）。

请你返回使 `nums1` 中所有数的和与 `nums2` 中所有数的和相等的最少操作次数。如果无法使两个数组的和相等，请返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以通过 3 次操作使 nums1 中所有数的和与 nums2 中所有数的和相等。以下数组下标都从 0 开始。
- 将 nums2[0] 变为 6 。 nums1 = [1,2,3,4,5,6], nums2 = [6,1,2,2,2,2] 。
- 将 nums1[5] 变为 1 。 nums1 = [1,2,3,4,5,1], nums2 = [6,1,2,2,2,2] 。
- 将 nums1[2] 变为 2 。 nums1 = [1,2,2,4,5,1], nums2 = [6,1,2,2,2,2] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,1,1,1,1,1,1], nums2 = [6]
<strong>输出:</strong> -1
<strong>解释:</strong> 没有办法减少 nums1 的和或者增加 nums2 的和使二者相等。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [6,6], nums2 = [1]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以通过 3 次操作使 nums1 中所有数的和与 nums2 中所有数的和相等。以下数组下标都从 0 开始。
- 将 nums1[0] 变为 2 。 nums1 = [2,6], nums2 = [1] 。
- 将 nums1[1] 变为 2 。 nums1 = [2,2], nums2 = [1] 。
- 将 nums2[0] 变为 4 。 nums1 = [2,2], nums2 = [4] 。
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* `1 <= nums1[i], nums2[i] <= 6`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.len() * 6 < nums2.len() || nums2.len() * 6 < nums1.len() {
            return -1;
        }

        let mut count1 = [0; 6];
        let mut count2 = [0; 6];
        let mut diff = 0;
        let mut ret = 0;

        for &x in &nums1 {
            count1[x as usize - 1] += 1;
            diff -= x;
        }
        for &x in &nums2 {
            count2[x as usize - 1] += 1;
            diff += x;
        }

        if diff < 0 {
            diff = -diff;
            (count1, count2) = (count2, count1);
        }

        for i in 0..5 {
            count1[i] += count2[5 - i];
            if (diff + 4 - i as i32) / (5 - i as i32) <= count1[i] {
                ret += (diff + 4 - i as i32) / (5 - i as i32);
                break;
            } else {
                diff -= count1[i] * (5 - i as i32);
                ret += count1[i];
            }
        }

        ret
    }
}
```
