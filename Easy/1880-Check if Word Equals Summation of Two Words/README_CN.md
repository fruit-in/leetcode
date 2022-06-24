# 1880. 检查某单词是否等于两单词之和
字母的 **字母值** 取决于字母在字母表中的位置，**从 0 开始** 计数。即，`'a' -> 0`、`'b' -> 1`、`'c' -> 2`，以此类推。

对某个由小写字母组成的字符串 `s` 而言，其 **数值** 就等于将 `s` 中每个字母的 **字母值** 按顺序 **连接** 并 **转换** 成对应整数。
* 例如，`s = "acb"` ，依次连接每个字母的字母值可以得到 `"021"` ，转换为整数得到 `21` 。

给你三个字符串 `firstWord`、`secondWord` 和 `targetWord` ，每个字符串都由从 `'a'` 到 `'j'` （**含** `'a'` 和 `'j'` ）的小写英文字母组成。

如果 `firstWord` 和 `secondWord` 的 **数值之和** 等于 `targetWord` 的数值，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> firstWord = "acb", secondWord = "cba", targetWord = "cdb"
<strong>输出:</strong> true
<strong>解释:</strong>
firstWord 的数值为 "acb" -> "021" -> 21
secondWord 的数值为 "cba" -> "210" -> 210
targetWord 的数值为 "cdb" -> "231" -> 231
由于 21 + 210 == 231 ，返回 true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> firstWord = "aaa", secondWord = "a", targetWord = "aab"
<strong>输出:</strong> false
<strong>解释:</strong>
firstWord 的数值为 "aaa" -> "000" -> 0
secondWord 的数值为 "a" -> "0" -> 0
targetWord 的数值为 "aab" -> "001" -> 1
由于 0 + 0 != 1 ，返回 false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> firstWord = "aaa", secondWord = "a", targetWord = "aaaa"
<strong>输出:</strong> true
<strong>解释:</strong>
firstWord 的数值为 "aaa" -> "000" -> 0
secondWord 的数值为 "a" -> "0" -> 0
targetWord 的数值为 "aaaa" -> "0000" -> 0
由于 0 + 0 == 0 ，返回 true
</pre>

#### 提示:
* `1 <= firstWord.length, secondWord.length, targetWord.length <= 8`
* `firstWord`、`secondWord` 和 `targetWord` 仅由从 `'a'` 到 `'j'` （含 `'a'` 和 `'j'` ）的小写英文字母组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let value1 = first_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let value2 = second_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let target = target_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);

        value1 + value2 == target
    }
}
```
