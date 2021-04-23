# 1758. Minimum Changes To Make Alternating Binary String
You are given a string `s` consisting only of the characters `'0'` and `'1'`. In one operation, you can change any `'0'` to `'1'` or vice versa.

The string is called alternating if no two adjacent characters are equal. For example, the string `"010"` is alternating, while the string `"0100"` is not.

Return *the **minimum** number of operations needed to make* `s` *alternating*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "0100"
<strong>Output:</strong> 1
<strong>Explanation:</strong> If you change the last character to '1', s will be "0101", which is alternating.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "10"
<strong>Output:</strong> 0
<strong>Explanation:</strong> s is already alternating.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1111"
<strong>Output:</strong> 2
<strong>Explanation:</strong> You need two operations to reach "0101" or "1010".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} s
# @return {Integer}
def min_operations(s)
  b = 0
  count = 0

  s.each_byte do |c|
    count += 1 if c != b + 48
    b = 1 - b
  end

  [count, s.size - count].min
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut b = b'0';
        let mut count = 0;

        for c in s.bytes() {
            if c != b {
                count += 1;
            }
            b = b'1' - b + b'0';
        }

        count.min(s.len() - count) as i32
    }
}
```
