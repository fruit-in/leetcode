# 1897. 重新分配字符使所有字符串都相等
给你一个字符串数组 `words`（下标 **从 0 开始** 计数）。

在一步操作中，需先选出两个 **不同** 下标 `i` 和 `j`，其中 `words[i]` 是一个非空字符串，接着将 `words[i]` 中的 任一 字符移动到 `words[j]` 中的 **任一** 位置上。

如果执行任意步操作可以使 `words` 中的每个字符串都相等，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["abc","aabc","bc"]
<strong>输出:</strong> true
<strong>解释:</strong> 将 words[1] 中的第一个 'a' 移动到 words[2] 的最前面。
使 words[1] = "abc" 且 words[2] = "abc" 。
所有字符串都等于 "abc" ，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["ab","a"]
<strong>输出:</strong> false
<strong>解释:</strong> 执行操作无法使所有字符串都相等。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = [0; 26];

        for word in &words {
            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }
        }

        count.iter().all(|&x| x % words.len() == 0)
    }
}
```
