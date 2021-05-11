# 467. 环绕字符串中唯一的子字符串
把字符串 `s` 看作是“abcdefghijklmnopqrstuvwxyz”的无限环绕字符串，所以 `s` 看起来是这样的："...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".

现在我们有了另一个字符串 `p` 。你需要的是找出 `s` 中有多少个唯一的 `p` 的非空子串，尤其是当你的输入是字符串 `p` ，你需要输出字符串 `s` 中 `p` 的不同的非空子串的数目。

**注意:** `p` 仅由小写的英文字母组成，`p` 的大小可能超过 10000。

#### 示例 1:
<pre>
<strong>输入:</strong> "a"
<strong>输出:</strong> 1
<strong>解释:</strong> 字符串 S 中只有一个"a"子字符。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "cac"
<strong>输出:</strong> 2
<strong>解释:</strong> 字符串 S 中的字符串“cac”只有两个子串“a”、“c”。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "zab"
<strong>输出:</strong> 6
<strong>解释:</strong> 在字符串 S 中有六个子串“z”、“a”、“b”、“za”、“ab”、“zab”。
</pre>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} p
# @return {Integer}
def find_substring_in_wrapround_string(p)
  p = p.bytes
  count = 1
  max_len = [0] * 26

  (0...p.size).each do |i|
    count = i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 ? 1 : count + 1
    (0...[26, count].min).each do |j|
      k = p[i + 1 - count + j] - 97
      max_len[k] = [max_len[k], count - j].max
    end
  end

  max_len.sum
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let p = p.as_bytes();
        let mut count = 1;
        let mut max_len = [0; 26];

        for i in 0..p.len() {
            if i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 {
                count = 1;
            } else {
                count += 1;
            }
            for j in 0..26.min(count) {
                let k = (p[i + 1 - count + j] - b'a') as usize;
                max_len[k] = max_len[k].max(count - j);
            }
        }

        max_len.iter().sum::<usize>() as i32
    }
}
```
