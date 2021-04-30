# 967. Numbers With Same Consecutive Differences
Return all **non-negative** integers of length `n` such that the absolute difference between every two consecutive digits is `k`.

Note that **every** number in the answer **must not** have leading zeros. For example, `01` has one leading zero and is invalid.

You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 7
<strong>Output:</strong> [181,292,707,818,929]
<strong>Explanation:</strong> Note that 070 is not a valid number, because it has leading zeroes.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, k = 1
<strong>Output:</strong> [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 2, k = 0
<strong>Output:</strong> [11,22,33,44,55,66,77,88,99]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 2, k = 2
<strong>Output:</strong> [13,20,24,31,35,42,46,53,57,64,68,75,79,86,97]
</pre>

#### Constraints:
* `2 <= n <= 9`
* `0 <= k <= 9`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {Integer[]}
def nums_same_consec_diff(n, k)
  nums = (1..9).to_a

  (2..n).each do |_|
    nums_ = []

    nums.each do |x|
      y = x % 10
      nums_.push(x * 10 + y + k) if y + k < 10
      nums_.push(x * 10 + y - k) if y - k >= 0 && k != 0
    end

    nums = nums_
  end

  nums
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut nums = (1..10).collect();

        for _ in 1..n {
            let mut nums_ = vec![];

            for x in nums {
                let y = x % 10;
                if y + k < 10 {
                    nums_.push(x * 10 + y + k);
                }
                if y - k >= 0 && k != 0 {
                    nums_.push(x * 10 + y - k);
                }
            }

            nums = nums_;
        }

        nums
    }
}
```
