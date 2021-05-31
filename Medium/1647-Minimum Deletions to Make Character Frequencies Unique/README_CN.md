# 1647. 字符频次唯一的最小删除次数
如果字符串 `s` 中 **不存在** 两个不同字符 **频次** 相同的情况，就称 `s` 是 **优质字符串** 。

给你一个字符串 `s`，返回使 `s` 成为 **优质字符串** 需要删除的 **最小** 字符数。

字符串中字符的 **频次** 是该字符在字符串中的出现次数。例如，在字符串 `"aab"` 中，`'a'` 的频次是 `2`，而 `'b'` 的频次是 `1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aab"
<strong>输出:</strong> 0
<strong>解释:</strong> s 已经是优质字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaabbbcc"
<strong>输出:</strong> 2
<strong>解释:</strong> 可以删除两个 'b' , 得到优质字符串 "aaabcc" 。
另一种方式是删除一个 'b' 和一个 'c' ，得到优质字符串 "aaabbc" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "ceabaacb"
<strong>输出:</strong> 2
<strong>解释:</strong> 可以删除两个 'c' 得到优质字符串 "eabaab" 。
注意，只需要关注结果字符串中仍然存在的字符。（即，频次为 0 的字符会忽略不计。）
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 仅含小写英文字母

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} s
# @return {Integer}
def min_deletions(s)
  counter = [0] * 26
  ret = 0

  s.each_byte { |c| counter[c - 97] += 1 }
  counter.sort!

  (25..1).step(-1).each do |i|
    if counter[i] <= counter[i - 1]
      ret += [counter[i - 1] - counter[i] + 1, counter[i - 1]].min
      counter[i - 1] = [counter[i] - 1, 0].max
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counter = vec![0; 26];
        let mut ret = 0;

        for c in s.bytes() {
            counter[(c - b'a') as usize] += 1;
        }
        counter.sort_unstable();

        for i in (1..26).rev() {
            if counter[i] <= counter[i - 1] {
                ret += (counter[i - 1] - counter[i] + 1).min(counter[i - 1]);
                counter[i - 1] = (counter[i] - 1).max(0);
            }
        }

        ret
    }
}
```
