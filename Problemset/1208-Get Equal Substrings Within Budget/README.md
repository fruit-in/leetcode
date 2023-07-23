# 1208. Get Equal Substrings Within Budget
You are given two strings `s` and `t` of the same length. You want to change `s` to `t`. Changing the `i`-th character of `s` to `i`-th character of `t` costs `|s[i] - t[i]|` that is, the absolute difference between the ASCII values of the characters.

You are also given an integer `maxCost`.

Return the maximum length of a substring of `s` that can be changed to be the same as the corresponding substring of `t` with a cost less than or equal to `maxCost`.

If there is no substring from `s` that can be changed to its corresponding substring from `t`, return `0`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcd", t = "bcdf", maxCost = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> "abc" of s can change to "bcd". That costs 3, so the maximum length is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcd", t = "cdef", maxCost = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> Each character in s costs 2 to change to charactor in t, so the maximum length is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "abcd", t = "acde", maxCost = 0
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can't make any change, so the maximum length is 1.
</pre>

#### Constraints:
* `1 <= s.length, t.length <= 10^5`
* `0 <= maxCost <= 10^6`
* `s` and `t` only contain lower case English letters.

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {String} s
# @param {String} t
# @param {Integer} max_cost
# @return {Integer}
def equal_substring(s, t, max_cost)
  s = s.bytes
  t = t.bytes
  i = -1
  cost = 0
  ret = 0

  (0...s.size).each do |j|
    cost += (s[j] - t[j]).abs
    while cost > max_cost
      i += 1
      cost -= (s[i] - t[i]).abs
    end
    ret = [ret, j - i].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = -1;
        let mut cost = 0;
        let mut ret = 0;

        for j in 0..s.len() {
            cost += (s[j] as i32 - t[j] as i32).abs();
            while cost > max_cost {
                i += 1;
                cost -= (s[i as usize] as i32 - t[i as usize] as i32).abs();
            }
            ret = ret.max(j as i32 - i);
        }

        ret
    }
}
```
