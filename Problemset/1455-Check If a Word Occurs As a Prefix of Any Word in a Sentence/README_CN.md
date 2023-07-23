# 1455. 检查单词是否为句中其他单词的前缀
给你一个字符串 `sentence` 作为句子并指定检索词为 `searchWord` ，其中句子由若干用 **单个空格** 分隔的单词组成。

请你检查检索词 `searchWord` 是否为句子 `sentence` 中任意单词的前缀。
* 如果 `searchWord` 是某一个单词的前缀，则返回句子 `sentence` 中该单词所对应的下标（**下标从 1 开始**）。
* 如果 `searchWord` 是多个单词的前缀，则返回匹配的第一个单词的下标（**最小下标**）。
* 如果 `searchWord` 不是任何单词的前缀，则返回 -1 。

字符串 `S` 的 「前缀」是 `S` 的任何前导连续子字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> sentence = "i love eating burger", searchWord = "burg"
<strong>输出:</strong> 4
<strong>解释:</strong> "burg" 是 "burger" 的前缀，而 "burger" 是句子中第 4 个单词。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sentence = "this problem is an easy problem", searchWord = "pro"
<strong>输出:</strong> 2
<strong>解释:</strong> "pro" 是 "problem" 的前缀，而 "problem" 是句子中第 2 个也是第 6 个单词，但是应该返回最小下标 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> sentence = "i am tired", searchWord = "you"
<strong>输出:</strong> -1
<strong>解释:</strong> "you" 不是句子中任何单词的前缀。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> sentence = "i use triple pillow", searchWord = "pill"
<strong>输出:</strong> 4
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> sentence = "hello from the other side", searchWord = "they"
<strong>输出:</strong> -1
</pre>

#### 提示:
* `1 <= sentence.length <= 100`
* `1 <= searchWord.length <= 10`
* `sentence` 由小写英文字母和空格组成。
* `searchWord` 由小写英文字母组成。
* 前缀就是紧密附着于词根的语素，中间不能插入其它成分，并且它的位置是固定的——-位于词根之前。（引用自 [前缀_百度百科](https://baike.baidu.com/item/%E5%89%8D%E7%BC%80) ）

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|s| s.starts_with(&search_word))
            .unwrap_or(-2_i32 as usize) as i32
            + 1
    }
}
```
