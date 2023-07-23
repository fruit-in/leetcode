# 567. 字符串的排列
给定两个字符串 `s1` 和 `s2`，写一个函数来判断 `s2` 是否包含 `s1` 的排列。

换句话说，第一个字符串的排列之一是第二个字符串的 **子串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "ab", s2 = "eidbaooo"
<strong>输出:</strong> True
<strong>解释:</strong> s2 包含 s1 的排列之一 ("ba").
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "ab", s2 = "eidboaoo"
<strong>输出:</strong> False
</pre>

#### 提示:
* 输入的字符串只包含小写字母
* 两个字符串的长度都在 `[1, 10,000]` 之间

## 题解 (Ruby)

### 1. 滑动窗口
```Ruby
# @param {String} s1
# @param {String} s2
# @return {Boolean}
def check_inclusion(s1, s2)
  return false if s1.size > s2.size

  count1 = [0] * 26
  count2 = [0] * 26

  (0...s1.size).each do |i|
    count1[s1[i].ord - 97] += 1
    count2[s2[i].ord - 97] += 1
  end

  (0..s2.size - s1.size).each do |i|
    return true if count1 == count2

    if i + s1.size < s2.size
      count2[s2[i].ord - 97] -= 1
      count2[s2[i + s1.size].ord - 97] += 1
    end
  end

  false
end
```

## 题解 (Rust)

### 1. 滑动窗口
```Rust
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];

        for i in 0..s1.len() {
            count1[(s1[i] - b'a') as usize] += 1;
            count2[(s2[i] - b'a') as usize] += 1;
        }

        for i in 0..=s2.len() - s1.len() {
            if count1 == count2 {
                return true;
            }
            if i + s1.len() < s2.len() {
                count2[(s2[i] - b'a') as usize] -= 1;
                count2[(s2[i + s1.len()] - b'a') as usize] += 1;
            }
        }

        false
    }
}
```
