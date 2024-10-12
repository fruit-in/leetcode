# 2452. 距离字典两次编辑以内的单词
给你两个字符串数组 `queries` 和 `dictionary` 。数组中所有单词都只包含小写英文字母，且长度都相同。

一次 **编辑** 中，你可以从 `queries` 中选择一个单词，将任意一个字母修改成任何其他字母。从 `queries` 中找到所有满足以下条件的字符串：**不超过** 两次编辑内，字符串与 `dictionary` 中某个字符串相同。

请你返回 `queries` 中的单词列表，这些单词距离 `dictionary` 中的单词 **编辑次数** 不超过 **两次** 。单词返回的顺序需要与 `queries` 中原本顺序相同。

#### 示例 1:
<pre>
<strong>输入:</strong> queries = ["word","note","ants","wood"], dictionary = ["wood","joke","moat"]
<strong>输出:</strong> ["word","note","wood"]
<strong>解释:</strong>
- 将 "word" 中的 'r' 换成 'o' ，得到 dictionary 中的单词 "wood" 。
- 将 "note" 中的 'n' 换成 'j' 且将 't' 换成 'k' ，得到 "joke" 。
- "ants" 需要超过 2 次编辑才能得到 dictionary 中的单词。
- "wood" 不需要修改（0 次编辑），就得到 dictionary 中相同的单词。
所以我们返回 ["word","note","wood"] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> queries = ["yes"], dictionary = ["not"]
<strong>输出:</strong> []
<strong>解释:</strong>
"yes" 需要超过 2 次编辑才能得到 "not" 。
所以我们返回空数组。
</pre>

#### 提示:
* `1 <= queries.length, dictionary.length <= 100`
* `n == queries[i].length == dictionary[j].length`
* `1 <= n <= 100`
* 所有 `queries[i]` 和 `dictionary[j]` 都只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ret = vec![];

        for wordq in &queries {
            for wordd in &dictionary {
                if wordq
                    .chars()
                    .zip(wordd.chars())
                    .filter(|(a, b)| a != b)
                    .count()
                    <= 2
                {
                    ret.push(wordq.clone());
                    break;
                }
            }
        }

        ret
    }
}
```
