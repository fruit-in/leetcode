# 2425. Bitwise XOR of All Pairings
You are given two **0-indexed** arrays, `nums1` and `nums2`, consisting of non-negative integers. There exists another array, `nums3`, which contains the bitwise XOR of **all pairings** of integers between `nums1` and `nums2` (every integer in `nums1` is paired with every integer in `nums2` **exactly once**).

Return *the **bitwise XOR** of all integers in* `nums3`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [2,1,3], nums2 = [10,2,5,0]
<strong>Output:</strong> 13
<strong>Explanation:</strong>
A possible nums3 array is [8,0,7,2,11,3,4,1,9,1,6,3].
The bitwise XOR of all these numbers is 13, so we return 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,2], nums2 = [3,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
All possible pairs of bitwise XORs are nums1[0] ^ nums2[0], nums1[0] ^ nums2[1], nums1[1] ^ nums2[0],
and nums1[1] ^ nums2[1].
Thus, one possible nums3 array is [2,5,1,6].
2 ^ 5 ^ 1 ^ 6 = 0, so we return 0.
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[j] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        match (nums1.len() % 2, nums2.len() % 2) {
            (0, 0) => 0,
            (0, 1) => nums1.iter().fold(0, |acc, x| acc ^ x),
            (1, 0) => nums2.iter().fold(0, |acc, x| acc ^ x),
            _ => nums1.iter().chain(nums2.iter()).fold(0, |acc, x| acc ^ x),
        }
    }
}
```
