# 187. 重复的DNA序列
所有 DNA 都由一系列缩写为 `'A'`，`'C'`，`'G'` 和 `'T'` 的核苷酸组成，例如：`"ACGAATTCCG"`。在研究 DNA 时，识别 DNA 中的重复序列有时会对研究非常有帮助。

编写一个函数来找出所有目标子串，目标子串的长度为 10，且在 DNA 字符串 `s` 中出现次数超过一次。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
<strong>输出:</strong> ["AAAAACCCCC","CCCCCAAAAA"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "AAAAAAAAAAAAA"
<strong>输出:</strong> ["AAAAAAAAAA"]
</pre>

#### 提示:
* <code>0 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 为 `'A'`、`'C'`、`'G'` 或 `'T'`

## 题解 (Ruby)

### 1. 哈希表
```Ruby
# @param {String} s
# @return {String[]}
def find_repeated_dna_sequences(s)
  counter = {}
  counter.default = 0

  (0..s.length - 10).each do |i|
    counter[s[i...i + 10]] += 1
  end

  counter.filter { |_, c| c > 1 }.keys
end
```

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;
use std::str;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut counter = HashMap::new();

        for w in s.windows(10) {
            *counter.entry(str::from_utf8(w).unwrap()).or_insert(0) += 1;
        }

        counter
            .into_iter()
            .filter(|(_, c)| *c > 1)
            .map(|(s, _)| s.to_string())
            .collect()
    }
}
```
