# 1784. Check if Binary String Has at Most One Segment of Ones
Given a binary string `s` **without leading zeros**, return `true` *if* `s` *contains **at most one contiguous segment of ones***. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1001"
<strong>Output:</strong> false
<strong>Explanation:</strong> The ones do not form a contiguous segment.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "110"
<strong>Output:</strong> true
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s[i]` is either `'0'` or `'1'`.
* `s[0]` is `'1'`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} s
# @return {Boolean}
def check_ones_segment(s)
  !s.include?('01')
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}
```
