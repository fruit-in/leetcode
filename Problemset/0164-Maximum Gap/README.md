# 164. Maximum Gap
Given an integer array `nums`, return *the maximum difference between two successive elements in its sorted form*. If the array contains less than two elements, return `0`.

You must write an algorithm that runs in linear time and uses linear extra space.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,6,9,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array contains less than 2 elements, therefore return 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums0 = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut x = 0xff;

        for i in (0..32).step_by(8) {
            let mut count = vec![0; 256];
            let mut nums1 = vec![0; nums0.len()];

            for j in 0..nums0.len() {
                count[(nums0[j] & x) >> i] += 1;
            }

            for j in 1..count.len() {
                count[j] += count[j - 1];
            }

            for j in (0..nums0.len()).rev() {
                count[(nums0[j] & x) >> i] -= 1;
                nums1[count[(nums0[j] & x) >> i]] = nums0[j];
            }

            nums0 = nums1;
            x <<= 8;
        }

        (1..nums0.len())
            .map(|i| nums0[i] - nums0[i - 1])
            .max()
            .unwrap_or(0) as i32
    }
}
```
