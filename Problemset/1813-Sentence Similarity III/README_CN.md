# 1813. 句子相似性 III
一个句子是由一些单词与它们之间的单个空格组成，且句子的开头和结尾没有多余空格。比方说，`"Hello World"` ，`"HELLO"` ，`"hello world hello world"` 都是句子。每个单词都 只 包含大写和小写英文字母。

如果两个句子 `sentence1` 和 `sentence2` ，可以通过往其中一个句子插入一个任意的句子（**可以是空句子**）而得到另一个句子，那么我们称这两个句子是 **相似的** 。比方说，`sentence1 = "Hello my name is Jane"` 且 `sentence2 = "Hello Jane"` ，我们可以往 `sentence2` 中 `"Hello"` 和 `"Jane"` 之间插入 `"my name is"` 得到 `sentence1` 。

给你两个句子 `sentence1` 和 `sentence2` ，如果 `sentence1` 和 `sentence2` 是相似的，请你返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> sentence1 = "My name is Haley", sentence2 = "My Haley"
<strong>输出:</strong> true
<strong>解释:</strong> 可以往 sentence2 中 "My" 和 "Haley" 之间插入 "name is" ，得到 sentence1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sentence1 = "of", sentence2 = "A lot of words"
<strong>输出:</strong> false
<strong>解释:</strong> 没法往这两个句子中的一个句子只插入一个句子就得到另一个句子。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> sentence1 = "Eating right now", sentence2 = "Eating"
<strong>输出:</strong> true
<strong>解释:</strong> 可以往 sentence2 的结尾插入 "right now" 得到 sentence1 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> sentence1 = "Luky", sentence2 = "Lucccky"
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= sentence1.length, sentence2.length <= 100`
* `sentence1` 和 `sentence2` 都只包含大小写英文字母和空格。
* `sentence1` 和 `sentence2` 中的单词都只由单个空格隔开。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut words1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let mut words2 = sentence2.split_whitespace().collect::<Vec<_>>();
        if words1.len() > words2.len() {
            let temp = words1;
            words1 = words2;
            words2 = temp;
        }

        while i < words1.len() && words1[i] == words2[i] {
            i += 1;
        }
        while j < words1.len() && words1[words1.len() - 1 - j] == words2[words2.len() - 1 - j] {
            j += 1;
        }

        i + j >= words1.len()
    }
}
```
