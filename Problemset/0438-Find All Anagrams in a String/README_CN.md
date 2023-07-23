# 438. 找到字符串中所有字母异位词
给定一个字符串 **s** 和一个非空字符串 **p**，找到 **s** 中所有是 **p** 的字母异位词的子串，返回这些子串的起始索引。

字符串只包含小写英文字母，并且字符串 **s** 和 **p** 的长度都不超过 20100。

#### 说明:
* 字母异位词指字母相同，但排列不同的字符串。
* 不考虑答案输出的顺序。

#### 示例 1:
<pre>
<strong>输入:</strong> s: "cbaebabacd" p: "abc"
<strong>输出:</strong> [0, 6]
<strong>解释:</strong>
起始索引等于 0 的子串是 "cba", 它是 "abc" 的字母异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的字母异位词。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s: "abab" p: "ab"
<strong>输出:</strong> [0, 1, 2]
<strong>解释:</strong>
起始索引等于 0 的子串是 "ab", 它是 "ab" 的字母异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的字母异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的字母异位词。
</pre>

## 题解 (Ruby)

### 1. 滑动窗口
```Ruby
# @param {String} s
# @param {String} p
# @return {Integer[]}
def find_anagrams(s, p)
  return [] if s.size < p.size

  count_p = [0] * 26
  count_s = [0] * 26
  ret = []

  (0...p.size).each do |i|
    count_p[p[i].ord - 97] += 1
    count_s[s[i].ord - 97] += 1
  end

  (0..s.size - p.size).each do |i|
    ret.push(i) if count_p == count_s
    if i + p.size < s.size
      count_s[s[i].ord - 97] -= 1
      count_s[s[i + p.size].ord - 97] += 1
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut count_p = [0; 26];
        let mut count_s = [0; 26];
        let mut ret = vec![];

        for i in 0..p.len() {
            count_p[(p[i] - b'a') as usize] += 1;
            count_s[(s[i] - b'a') as usize] += 1;
        }

        for i in 0..=s.len() - p.len() {
            if count_p == count_s {
                ret.push(i as i32);
            }
            if i + p.len() < s.len() {
                count_s[(s[i] - b'a') as usize] -= 1;
                count_s[(s[i + p.len()] - b'a') as usize] += 1;
            }
        }

        ret
    }
}
```
