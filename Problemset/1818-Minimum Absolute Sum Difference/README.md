# 1818. Minimum Absolute Sum Difference
You are given two positive integer arrays `nums1` and `nums2`, both of length `n`.

The **absolute sum difference** of arrays `nums1` and `nums2` is defined as the **sum** of `|nums1[i] - nums2[i]|` for each `0 <= i < n` (**0-indexed**).

You can replace **at most one** element of `nums1` with **any** other element in `nums1` to **minimize** the absolute sum difference.

Return the *minimum absolute sum difference **after** replacing at most one element in the array `nums1`*. Since the answer may be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

`|x|` is defined as:
* `x` if `x >= 0`, or
* `-x` if `x < 0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,7,5], nums2 = [2,3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are two possible optimal solutions:
- Replace the second element with the first: [1,7,5] => [1,1,5], or
- Replace the second element with the third: [1,7,5] => [1,5,5].
Both will yield an absolute sum difference of |1-2| + (|1-3| or |5-3|) + |5-5| = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [2,4,6,8,10], nums2 = [2,4,6,8,10]
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums1 is equal to nums2 so no replacement is needed. This will result in an
absolute sum difference of 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [1,10,4,4,2,7], nums2 = [9,3,5,1,7,4]
<strong>Output:</strong> 20
<strong>Explanation:</strong> Replace the first element with the second: [1,10,4,4,2,7] => [10,10,4,4,2,7].
This yields an absolute sum difference of |10-9| + |10-3| + |4-5| + |4-1| + |2-7| + |7-4| = 20
</pre>

#### Constraints:
* `n == nums1.length`
* `n == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums1[i], nums2[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut x = 0;
        let mut nums = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        nums.sort_unstable();

        for i in 0..nums.len() {
            let y = (nums[i].1 - nums[i].0).abs();
            let z = match nums.binary_search_by_key(&nums[i].1, |&(a, _)| a) {
                Ok(_) => 0,
                Err(0) => nums[0].0 - nums[i].1,
                Err(j) if j == nums.len() => nums[i].1 - nums[j - 1].0,
                Err(j) => (nums[i].1 - nums[j - 1].0).min(nums[j].0 - nums[i].1),
            };

            sum = (sum + y) % 1_000_000_007;
            x = x.max(y - z);
        }

        (sum - x).rem_euclid(1_000_000_007)
    }
}
```
