# 2546. Apply Bitwise Operations to Make Strings Equal
You are given two **0-indexed binary** strings `s` and `target` of the same length `n`. You can do the following operation on `s` **any** number of times:
* Choose two **different** indices `i` and `j` where `0 <= i, j < n`.
* Simultaneously, replace `s[i]` with (`s[i]` **OR** `s[j]`) and `s[j]` with (`s[i]` **XOR** `s[j]`).

For example, if `s = "0110"`, you can choose `i = 0` and `j = 2`, then simultaneously replace `s[0]` with (`s[0]` **OR** `s[2]` = `0` **OR** `1` = `1`), and `s[2]` with (`s[0]` **XOR** `s[2]` = `0` **XOR** `1` = `1`), so we will have `s = "1110"`.

Return `true` *if you can make the string* `s` *equal to* `target`*, or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1010", target = "0110"
<strong>Output:</strong> true
<strong>Explanation:</strong> We can do the following operations:
- Choose i = 2 and j = 0. We have now s = "0010".
- Choose i = 2 and j = 1. We have now s = "0110".
Since we can make s equal to target, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "11", target = "00"
<strong>Output:</strong> false
<strong>Explanation:</strong> It is not possible to make s equal to target with any number of operations.
</pre>

#### Constraints:
* `n == s.length == target.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `s` and `target` consist of only the digits `0` and `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.contains('1') == target.contains('1')
    }
}
```
