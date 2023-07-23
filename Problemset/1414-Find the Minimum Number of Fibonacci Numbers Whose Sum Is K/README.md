# 1414. Find the Minimum Number of Fibonacci Numbers Whose Sum Is K
Given an integer `k`, *return the minimum number of Fibonacci numbers whose sum is equal to* `k`. The same Fibonacci number can be used multiple times.

The Fibonacci numbers are defined as:
* <code>F<sub>1</sub> = 1</code>
* <code>F<sub>2</sub> = 1</code>
* <code>F<sub>n</sub> = F<sub>n-1</sub> + F<sub>n-2</sub></code> for `n > 2`.

It is guaranteed that for the given constraints we can always find such Fibonacci numbers that sum up to `k`.

#### Example 1:
<pre>
<strong>Input:</strong> k = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> The Fibonacci numbers are: 1, 1, 2, 3, 5, 8, 13, ...
For k = 7 we can use 2 + 5 = 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 10
<strong>Output:</strong> 2
<strong>Explanation:</strong> For k = 10 we can use 2 + 8 = 10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 19
<strong>Output:</strong> 3
<strong>Explanation:</strong> For k = 19 we can use 1 + 5 + 13 = 19.
</pre>

#### Constraints:
* `1 <= k <= 10^9`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer} k
# @return {Integer}
def find_min_fibonacci_numbers(k)
  nums = [1, 1]
  ret = 0

  nums.push(nums[-2] + nums[-1]) while nums[-1] < k

  while k > 0
    nums.pop while nums[-1] > k
    k -= nums[-1]
    ret += 1
  end

  ret
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut nums = vec![1, 1];
        let mut i = 1;
        let mut ret = 0;

        while nums[i] < k {
            nums.push(nums[i - 1] + nums[i]);
            i += 1;
        }

        while k > 0 {
            while nums[i] > k {
                i -= 1;
            }
            k -= nums[i];
            ret += 1;
        }

        ret
    }
}
```
