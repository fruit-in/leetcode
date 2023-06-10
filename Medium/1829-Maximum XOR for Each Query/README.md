# 1829. Maximum XOR for Each Query
You are given a **sorted** array `nums` of `n` non-negative integers and an integer `maximumBit`. You want to perform the following query `n` **times**:

1. Find a non-negative integer <code>k < 2<sup>maximumBit</sup></code> such that `nums[0] XOR nums[1] XOR ... XOR nums[nums.length-1] XOR k` is **maximized**. `k` is the answer to the <code>i<sup>th</sup></code> query.
2. Remove the **last** element from the current array `nums`.

Return *an array* `answer`, *where* `answer[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,1,3], maximumBit = 2
<strong>Output:</strong> [0,3,2,3]
<strong>Explanation:</strong> The queries are answered as follows:
1st query: nums = [0,1,1,3], k = 0 since 0 XOR 1 XOR 1 XOR 3 XOR 0 = 3.
2nd query: nums = [0,1,1], k = 3 since 0 XOR 1 XOR 1 XOR 3 = 3.
3rd query: nums = [0,1], k = 2 since 0 XOR 1 XOR 2 = 3.
4th query: nums = [0], k = 3 since 0 XOR 3 = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,4,7], maximumBit = 3
<strong>Output:</strong> [5,2,6,5]
<strong>Explanation:</strong> The queries are answered as follows:
1st query: nums = [2,3,4,7], k = 5 since 2 XOR 3 XOR 4 XOR 7 XOR 5 = 7.
2nd query: nums = [2,3,4], k = 2 since 2 XOR 3 XOR 4 XOR 2 = 7.
3rd query: nums = [2,3], k = 6 since 2 XOR 3 XOR 6 = 7.
4th query: nums = [2], k = 5 since 2 XOR 5 = 7.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0,1,2,2,5,7], maximumBit = 3
<strong>Output:</strong> [4,3,6,4,6,7]
</pre>

#### Constraints:
* `nums.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= maximumBit <= 20`
* <code>0 <= nums[i] < 2<sup>maximumBit</sup></code>
* `nums` is sorted in **ascending** order.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut answer = vec![(1 << maximum_bit) - 1; nums.len()];

        for i in 0..nums.len() {
            answer[i] ^= xor;
            xor ^= nums[nums.len() - 1 - i];
        }

        answer
    }
}
```
