# 1234. Replace the Substring for Balanced String
You are given a string containing only 4 kinds of characters `'Q'`, `'W'`, `'E'` and `'R'`.

A string is said to be **balanced** if each of its characters appears `n/4` times where `n` is the length of the string.

Return the minimum length of the substring that can be replaced with **any** other string of the same length to make the original string `s` **balanced**.

Return 0 if the string is already **balanced**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "QWER"
<strong>Output:</strong> 0
<strong>Explanation:</strong> s is already balanced.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "QQWE"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We need to replace a 'Q' to 'R', so that "RQWE" (or "QRWE") is balanced.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "QQQW"
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can replace the first "QQ" to "ER".
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "QQQQ"
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can replace the last 3 'Q' to make s = "QWER".
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `s.length` is a multiple of `4`
* `s` contains only `'Q'`, `'W'`, `'E'` and `'R'`.

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {String} s
# @return {Integer}
def balanced_string(s)
  count = {}
  count.default = 0
  l = 0
  ret = s.size

  s.each_char { |c| count[c] += 1 }

  (0..s.size).each do |r|
    while l <= r && 4 * count.values.max - count.values.sum <= r - l
      ret = [ret, r - l].min
      count[s[[l, s.size - 1].min]] += 1
      l += 1
    end
    count[s[[r, s.size - 1].min]] -= 1
  end

  ret
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 4];
        let mut l = 0;
        let mut ret = s.len();

        for c in s {
            match c {
                b'Q' => count[0] += 1,
                b'W' => count[1] += 1,
                b'E' => count[2] += 1,
                _ => count[3] += 1,
            }
        }

        for r in 0..=s.len() {
            while l <= r && 4 * count.iter().max().unwrap() - count.iter().sum::<usize>() <= r - l {
                ret = ret.min(r - l);
                match s[l.min(s.len() - 1)] {
                    b'Q' => count[0] += 1,
                    b'W' => count[1] += 1,
                    b'E' => count[2] += 1,
                    _ => count[3] += 1,
                }
                l += 1;
            }
            match s[r.min(s.len() - 1)] {
                b'Q' => count[0] -= 1,
                b'W' => count[1] -= 1,
                b'E' => count[2] -= 1,
                _ => count[3] -= 1,
            }
        }

        ret as i32
    }
}
```
