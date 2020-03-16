# 1324. 竖直打印单词
给你一个字符串 ```s```。请你按照单词在 ```s``` 中的出现顺序将它们全部竖直返回。
单词应该以字符串列表的形式返回，必要时用空格补位，但输出尾部的空格需要删除（不允许尾随空格）。
每个单词只能放在一列上，每一列中也只能有一个单词。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "HOW ARE YOU"
<strong>输出:</strong> ["HAY","ORO","WEU"]
<strong>解释:</strong> 每个单词都应该竖直打印。 
 "HAY"
 "ORO"
 "WEU"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "TO BE OR NOT TO BE"
<strong>输出:</strong> ["TBONTB","OEROOE","   T"]
<strong>解释:</strong> 题目允许使用空格补位，但不允许输出末尾出现空格。
"TBONTB"
"OEROOE"
"   T"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "CONTEST IS COMING"
<strong>输出:</strong> ["CIC","OSO","N M","T I","E N","S G","T"]
</pre>

#### 提示:
* ```1 <= s.length <= 200```
* ```s``` 仅含大写英文字母。
* 题目数据保证两个单词之间只有一个空格。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def printVertically(self, s: str) -> List[str]:
        words = s.split()
        max_len = max(len(word) for word in words)
        v_words = [''] * max_len

        for i in range(max_len):
            for word in words:
                v_words[i] += word[i] if len(word) > i else ' '

        return [word.rstrip() for word in v_words]
```
