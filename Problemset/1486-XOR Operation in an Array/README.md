# 1486. XOR Operation in an Array
Given an integer `n` and an integer `start`.

Define an array `nums` where `nums[i] = start + 2*i` (0-indexed) and `n == nums.length`.

Return the bitwise XOR of all elements of `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5, start = 0
<strong>Output:</strong> 8
<strong>Explanation:</strong> Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
Where "^" corresponds to bitwise XOR operator.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, start = 3
<strong>Output:</strong> 8
<strong>Explanation:</strong> Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 1, start = 7
<strong>Output:</strong> 7
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 10, start = 5
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `1 <= n <= 1000`
* `0 <= start <= 1000`
* `n == nums.length`

## Solutions (Ruby)

### 1. Simulation
```Ruby
# @param {Integer} n
# @param {Integer} start
# @return {Integer}
def xor_operation(n, start)
    ret = 0

    (0...n).each do |i|
        ret ^= start + 2 * i
    end

    return ret
end
```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}
```
