# 1208. 尽可能使字符串相等
给你两个长度相同的字符串，`s` 和 `t`。

将 `s` 中的第 `i` 个字符变到 `t` 中的第 `i` 个字符需要 `|s[i] - t[i]|` 的开销（开销可能为 0），也就是两个字符的 ASCII 码值的差的绝对值。

用于变更字符串的最大预算是 `maxCost`。在转化字符串时，总开销应当小于等于该预算，这也意味着字符串的转化可能是不完全的。

如果你可以将 `s` 的子字符串转化为它在 `t` 中对应的子字符串，则返回可以转化的最大长度。

如果 `s` 中没有子字符串可以转化成 `t` 中对应的子字符串，则返回 `0`。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcd", t = "bcdf", maxCost = 3
<strong>输出:</strong> 3
<strong>解释:</strong> s 中的 "abc" 可以变为 "bcd"。开销为 3，所以最大长度为 3。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcd", t = "cdef", maxCost = 3
<strong>输出:</strong> 1
<strong>解释:</strong> s 中的任一字符要想变成 t 中对应的字符，其开销都是 2。因此，最大长度为 1。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "abcd", t = "acde", maxCost = 0
<strong>输出:</strong> 1
<strong>解释:</strong> a -> a, cost = 0，字符串未发生变化，所以最大长度为 1。
</pre>

#### 提示:
* `1 <= s.length, t.length <= 10^5`
* `0 <= maxCost <= 10^6`
* `s` 和 `t` 都只含小写英文字母。

## 题解 (Ruby)

### 1. 双指针
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

## 题解 (Rust)

### 1. 双指针
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
