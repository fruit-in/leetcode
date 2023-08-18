# 1807. 替换字符串中的括号内容
给你一个字符串 `s` ，它包含一些括号对，每个括号中包含一个 **非空** 的键。

* 比方说，字符串 `"(name)is(age)yearsold"` 中，有 **两个** 括号对，分别包含键 `"name"` 和 `"age"` 。

你知道许多键对应的值，这些关系由二维字符串数组 `knowledge` 表示，其中 <code>knowledge[i] = [key<sub>i</sub>, value<sub>i</sub>]</code> ，表示键 <code>key<sub>i</sub></code> 对应的值为 <code>value<sub>i</sub></code> 。

你需要替换 **所有** 的括号对。当你替换一个括号对，且它包含的键为 <code>key<sub>i</sub></code> 时，你需要：

* 将 <code>key<sub>i</sub></code> 和括号用对应的值 <code>value<sub>i</sub></code> 替换。
* 如果从 `knowledge` 中无法得知某个键对应的值，你需要将 <code>key<sub>i</sub></code> 和括号用问号 `"?"` 替换（不需要引号）。

`knowledge` 中每个键最多只会出现一次。`s` 中不会有嵌套的括号。

请你返回替换 **所有** 括号对后的结果字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "(name)is(age)yearsold", knowledge = [["name","bob"],["age","two"]]
<strong>输出:</strong> "bobistwoyearsold"
<strong>解释:</strong>
键 "name" 对应的值为 "bob" ，所以将 "(name)" 替换为 "bob" 。
键 "age" 对应的值为 "two" ，所以将 "(age)" 替换为 "two" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "hi(name)", knowledge = [["a","b"]]
<strong>输出:</strong> "hi?"
<strong>解释:</strong> 由于不知道键 "name" 对应的值，所以用 "?" 替换 "(name)" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "(a)(a)(a)aaa", knowledge = [["a","yes"]]
<strong>输出:</strong> "yesyesyesaaa"
<strong>解释:</strong> 相同的键在 s 中可能会出现多次。
键 "a" 对应的值为 "yes" ，所以将所有的 "(a)" 替换为 "yes" 。
注意，不在括号里的 "a" 不需要被替换。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* <code>0 <= knowledge.length <= 10<sup>5</sup></code>
* `knowledge[i].length == 2`
* <code>1 <= key<sub>i</sub>.length, value<sub>i</sub>.length <= 10</code>
* `s` 只包含小写英文字母和圆括号 `'('` 和 `')'` 。
* `s` 中每一个左圆括号 `'('` 都有对应的右圆括号 `')'` 。
* `s` 中每对括号内的键都不会为空。
* `s` 中不会有嵌套括号对。
* <code>key<sub>i</sub></code> 和 <code>value<sub>i</sub></code> 只包含小写英文字母。
* `knowledge` 中的 <code>key<sub>i</sub></code> 不会重复。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        knowledge = dict(knowledge)
        key = ""
        ret = ""

        for ch in s:
            if ch == ')':
                ret += knowledge.get(key[1:], "?")
                key = ""
            elif key != "" or ch == '(':
                key += ch
            else:
                ret += ch

        return ret
```
