# 1646. Get Maximum in Generated Array
You are given an integer `n`. An array `nums` of length `n + 1` is generated in the following way:
* `nums[0] = 0`
* `nums[1] = 1`
* `nums[2 * i] = nums[i]` when `2 <= 2 * i <= n`
* `nums[2 * i + 1] = nums[i] + nums[i + 1]` when `2 <= 2 * i + 1 <= n`

Return *the **maximum** integer in the array* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> 3
<strong>Explanation:</strong> According to the given rules:
  nums[0] = 0
  nums[1] = 1
  nums[(1 * 2) = 2] = nums[1] = 1
  nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
  nums[(2 * 2) = 4] = nums[2] = 1
  nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
  nums[(3 * 2) = 6] = nums[3] = 2
  nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> According to the given rules, the maximum between nums[0], nums[1], and nums[2] is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> According to the given rules, the maximum between nums[0], nums[1], nums[2], and nums[3] is 2.
</pre>

#### Constraints:
* `0 <= n <= 100`

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums = vec![0; n as usize + 1];

        for j in 1..nums.len() {
            nums[j] = match j / 2 {
                0 => 1,
                i if j % 2 == 0 => nums[i],
                i => nums[i] + nums[i + 1],
            };
        }

        nums.into_iter().max().unwrap()
    }
}
```
