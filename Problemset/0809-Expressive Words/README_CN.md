# 809. 情感丰富的文字
有时候人们会用重复写一些字母来表示额外的感受，比如 `"hello" -> "heeellooo"`, `"hi" -> "hiii"`。我们将相邻字母都相同的一串字符定义为相同字母组，例如："h", "eee", "ll", "ooo"。

对于一个给定的字符串 S ，如果另一个单词能够通过将一些字母组扩张从而使其和 S 相同，我们将这个单词定义为可扩张的（stretchy）。扩张操作定义如下：选择一个字母组（包含字母 `c` ），然后往其中添加相同的字母 `c` 使其长度达到 3 或以上。

例如，以 "hello" 为例，我们可以对字母组 "o" 扩张得到 "hellooo"，但是无法以同样的方法得到 "helloo" 因为字母组 "oo" 长度小于 3。此外，我们可以进行另一种扩张 "ll" -> "lllll" 以获得 "helllllooo"。如果 `S = "helllllooo"`，那么查询词 "hello" 是可扩张的，因为可以对它执行这两种扩张操作使得 `query = "hello" -> "hellooo" -> "helllllooo" = S`。

输入一组查询单词，输出其中可扩张的单词数量。

#### 示例:
<pre>
<strong>输入:</strong>
S = "heeellooo"
words = ["hello", "hi", "helo"]
<strong>输出:</strong> 1
<strong>解释:</strong>
我们能通过扩张 "hello" 的 "e" 和 "o" 来得到 "heeellooo"。
我们不能通过扩张 "helo" 来得到 "heeellooo" 因为 "ll" 的长度小于 3 。
</pre>

#### 提示:
* `0 <= len(S) <= 100`。
* `0 <= len(words) <= 100`。
* `0 <= len(words[i]) <= 100`。
* `S` 和所有在 `words` 中的单词都只由小写字母组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = Self::group(&s);

        words
            .iter()
            .map(|w| Self::group(w))
            .filter(|w| {
                s.len() == w.len()
                    && s.iter().zip(w.iter()).all(|(&(c0, x), &(c1, y))| {
                        c0 == c1 && ((y == 1 && x != 2) || (y != 1 && x >= y))
                    })
            })
            .count() as i32
    }

    fn group(s: &str) -> Vec<(char, i32)> {
        let mut ret = vec![];

        for c0 in s.chars() {
            match ret.last_mut() {
                Some((c1, x)) if c0 == *c1 => *x += 1,
                _ => ret.push((c0, 1)),
            }
        }

        ret
    }
}
```
