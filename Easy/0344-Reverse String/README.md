# 344. Reverse String
Write a function that reverses a string. The input string is given as an array of characters <code>char[]</code>.

Do not allocate extra space for another array, you must do this by **modifying the input array in-place** with O(1) extra memory.

You may assume all the characters consist of printable ascii characters.

#### Example 1:
<pre>
<strong>Input:</strong> ["h","e","l","l","o"]
<strong>Output:</strong> ["o","l","l","e","h"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["H","a","n","n","a","h"]
<strong>Output:</strong> ["h","a","n","n","a","H"]
</pre>

#### Note:

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() > 0 {
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
    }
}
```
