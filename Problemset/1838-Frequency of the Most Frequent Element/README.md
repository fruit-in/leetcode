# 1838. Frequency of the Most Frequent Element
The **frequency** of an element is the number of times it occurs in an array.

You are given an integer array `nums` and an integer `k`. In one operation, you can choose an index of `nums` and increment the element at that index by `1`.

Return *the **maximum possible frequency** of an element after performing **at most*** `k` *operations*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,4], k = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong> Increment the first element three times and the second element two times to make nums = [4,4,4].
4 has a frequency of 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,4,8,13], k = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are multiple optimal solutions:
- Increment the first element three times to make nums = [4,4,8,13]. 4 has a frequency of 2.
- Increment the second element four times to make nums = [1,8,8,13]. 8 has a frequency of 2.
- Increment the third element five times to make nums = [1,4,13,13]. 13 has a frequency of 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,9,6], k = 2
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut sum;
        let mut ret = 1;

        nums.sort_unstable();
        sum = nums[0] as i64;

        for j in 1..nums.len() {
            while (j - i) as i64 * nums[j] as i64 - sum > k as i64 {
                sum -= nums[i] as i64;
                i += 1;
            }

            sum += nums[j] as i64;
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
