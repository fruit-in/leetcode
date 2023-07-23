# 390. Elimination Game
You have a list `arr` of all integers in the range `[1, n]` sorted in a strictly increasing order. Apply the following algorithm on `arr`:

* Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
* Repeat the previous step again, but this time from right to left, remove the rightmost number and every other number from the remaining numbers.
* Keep repeating the steps again, alternating left to right and right to left, until a single number remains.

Given the integer `n`, return *the last number that remains in* `arr`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 9
<strong>Output:</strong> 6
<strong>Explanation:</strong>
arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]
arr = [2, 4, 6, 8]
arr = [2, 6]
arr = [6]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining_rev(n / 2)
    }

    pub fn last_remaining_rev(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        2 * Self::last_remaining(n / 2) - 1 + n % 2
    }
}
```
