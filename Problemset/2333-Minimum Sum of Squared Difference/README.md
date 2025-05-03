# 2333. Minimum Sum of Squared Difference
You are given two positive **0-indexed** integer arrays `nums1` and `nums2`, both of length `n`.

The **sum of squared difference** of arrays `nums1` and `nums2` is defined as the **sum** of <code>(nums1[i] - nums2[i])<sup>2</sup></code> for each `0 <= i < n`.

You are also given two positive integers `k1` and `k2`. You can modify any of the elements of `nums1` by `+1` or `-1` at most `k1` times. Similarly, you can modify any of the elements of `nums2` by `+1` or `-1` at most `k2` times.

Return *the minimum **sum of squared difference** after modifying array* `nums1` *at most* `k1` *times and modifying array* `nums2` *at most* `k2` *times*.

**Note:** You are allowed to modify the array elements to become negative integers.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,2,3,4], nums2 = [2,10,20,19], k1 = 0, k2 = 0
<strong>Output:</strong> 579
<strong>Explanation:</strong> The elements in nums1 and nums2 cannot be modified because k1 = 0 and k2 = 0.
The sum of square difference will be: (1 - 2)2 + (2 - 10)2 + (3 - 20)2 + (4 - 19)2 = 579.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,4,10,12], nums2 = [5,8,6,9], k1 = 1, k2 = 1
<strong>Output:</strong> 43
<strong>Explanation:</strong> One way to obtain the minimum sum of square difference is:
- Increase nums1[0] once.
- Increase nums2[2] once.
The minimum of the sum of square difference will be:
(2 - 5)2 + (4 - 8)2 + (10 - 7)2 + (12 - 9)2 = 43.
Note that, there are other ways to obtain the minimum of the sum of square difference, but there is no way to obtain a sum smaller than 43.
</pre>

#### Constraints:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[i] <= 10<sup>5</sup></code>
* <code>0 <= k1, k2 <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut k = (k1 + k2) as i64;
        let mut diffs = (0..nums1.len())
            .map(|i| (nums1[i] - nums2[i]).abs() as i64)
            .collect::<Vec<_>>();
        let mut count = 1;

        diffs.push(0);
        diffs.sort_unstable();

        for i in (0..diffs.len()).rev() {
            if diffs[i] == 0 {
                return 0;
            }

            if (diffs[i] - diffs[i - 1]) * count <= k {
                k -= (diffs[i] - diffs[i - 1]) * count;
                count += 1;
            } else {
                let tmp = diffs[i] - k / count;
                k -= (diffs[i] - tmp) * count;
                return count * tmp * tmp - 2 * k * tmp
                    + k
                    + (0..i).map(|j| diffs[j] * diffs[j]).sum::<i64>();
            }
        }

        unreachable!()
    }
}
```
