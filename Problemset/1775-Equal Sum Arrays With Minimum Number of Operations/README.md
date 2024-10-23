# 1775. Equal Sum Arrays With Minimum Number of Operations
You are given two arrays of integers `nums1` and `nums2`, possibly of different lengths. The values in the arrays are between `1` and `6`, inclusive.

In one operation, you can change any integer's value in **any** of the arrays to **any** value between `1` and `6`, inclusive.

Return *the minimum number of operations required to make the sum of values in* `nums1` *equal to the sum of values in* `nums2`. Return `-1` if it is not possible to make the sum of the two arrays equal.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [6,1,2,2,2,2].
- Change nums1[5] to 1. nums1 = [1,2,3,4,5,1], nums2 = [6,1,2,2,2,2].
- Change nums1[2] to 2. nums1 = [1,2,2,4,5,1], nums2 = [6,1,2,2,2,2].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,1,1,1,1,1,1], nums2 = [6]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to decrease the sum of nums1 or to increase the sum of nums2 to make them equal.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [6,6], nums2 = [1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums1[0] to 2. nums1 = [2,6], nums2 = [1].
- Change nums1[1] to 2. nums1 = [2,2], nums2 = [1].
- Change nums2[0] to 4. nums1 = [2,2], nums2 = [4].
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* `1 <= nums1[i], nums2[i] <= 6`

## Solutions (Rust)

### 1. Solution
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
