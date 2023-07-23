# 1451. 重新排列句子中的单词
「句子」是一个用空格分隔单词的字符串。给你一个满足下述格式的句子 `text` :
* 句子的首字母大写
* `text` 中的每个单词都用单个空格分隔。

请你重新排列 `text` 中的单词，使所有单词按其长度的升序排列。如果两个单词的长度相同，则保留其在原句子中的相对顺序。

请同样按上述格式返回新的句子。

#### 示例 1:
<pre>
<b>输入:</b> text = "Leetcode is cool"
<b>输出:</b> "Is cool leetcode"
<b>解释:</b> 句子中共有 3 个单词，长度为 8 的 "Leetcode" ，长度为 2 的 "is" 以及长度为 4 的 "cool" 。
输出需要按单词的长度升序排列，新句子中的第一个单词首字母需要大写。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> text = "Keep calm and code on"
<b>输出:</b> "On and keep calm code"
<b>解释:</b> 输出的排序情况如下：
"On" 2 个字母。
"and" 3 个字母。
"keep" 4 个字母，因为存在长度相同的其他单词，所以它们之间需要保留在原句子中的相对顺序。
"calm" 4 个字母。
"code" 4 个字母。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> text = "To be or not to be"
<b>输出:</b> "To be or to be not"
</pre>

#### 提示:
* `text` 以大写字母开头，然后包含若干小写字母以及单词间的单个空格。
* `1 <= text.length <= 10^5`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} text
# @return {String}
def arrange_words(text)
    words = text.downcase.split(' ')
    words.sort_by! {|word| word.length}
    words[0].capitalize!

    return words.join(' ')
end
```
