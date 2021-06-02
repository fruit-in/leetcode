# 984. String Without AAA or BBB
Given two integers `a` and `b`, return **any** string `s` such that:
* `s` has length `a + b` and contains exactly `a` `'a'` letters, and exactly `b` `'b'` letters,
* The substring `'aaa'` does not occur in `s`, and
* The substring `'bbb'` does not occur in `s`.

#### Example 1:
<pre>
<strong>Input:</strong> a = 1, b = 2
<strong>Output:</strong> "abb"
<strong>Explanation:</strong> "abb", "bab" and "bba" are all correct answers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 4, b = 1
<strong>Output:</strong> "aabaa"
</pre>

#### Constraints:
* `0 <= a, b <= 100`
* It is guaranteed such an `s` exists for the given `a` and `b`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} a
# @param {Integer} b
# @return {String}
def str_without3a3b(a, b)
  more_ch, less_ch = a > b ? %w[a b] : %w[b a]
  more_cnt = [a, b].max
  less_cnt = [a, b].min

  part0 = more_ch * 2 + less_ch
  part1 = more_ch + less_ch
  part2 = more_ch

  x = [more_cnt - less_cnt, less_cnt].min
  y = [2 * less_cnt - more_cnt, 0].max
  z = [more_cnt - 2 * less_cnt, 0].max

  part0 * x + part1 * y + part2 * z
end
```
