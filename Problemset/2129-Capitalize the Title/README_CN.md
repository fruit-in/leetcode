# 2129. 将标题首字母大写
给你一个字符串 `title` ，它由单个空格连接一个或多个单词组成，每个单词都只包含英文字母。请你按以下规则将每个单词的首字母 **大写** ：
* 如果单词的长度为 `1` 或者 `2` ，所有字母变成小写。
* 否则，将单词首字母大写，剩余字母变成小写。

请你返回 **大写后** 的 `title` 。

#### 示例 1:
<pre>
<strong>输入:</strong> title = "capiTalIze tHe titLe"
<strong>输出:</strong> "Capitalize The Title"
<strong>解释:</strong>
由于所有单词的长度都至少为 3 ，将每个单词首字母大写，剩余字母变为小写。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> title = "First leTTeR of EACH Word"
<strong>输出:</strong> "First Letter of Each Word"
<strong>解释:</strong>
单词 "of" 长度为 2 ，所以它保持完全小写。
其他单词长度都至少为 3 ，所以其他单词首字母大写，剩余字母小写。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> title = "i lOve leetcode"
<strong>输出:</strong> "i Love Leetcode"
<strong>解释:</strong>
单词 "i" 长度为 1 ，所以它保留小写。
其他单词长度都至少为 3 ，所以其他单词首字母大写，剩余字母小写。
</pre>

#### 提示:
* `1 <= title.length <= 100`
* `title` 由单个空格隔开的单词组成，且不含有任何前导或后缀空格。
* 每个单词由大写和小写英文字母组成，且都是 **非空** 的。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def capitalizeTitle(self, title: str) -> str:
        return ' '.join(w.lower() if len(w) < 3 else w.capitalize() for w in title.split())
```
