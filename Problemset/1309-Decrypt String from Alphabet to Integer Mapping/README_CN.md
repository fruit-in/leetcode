# 1309. 解码字母到整数映射
给你一个字符串 ```s```，它由数字（```'0'``` - ```'9'```）和 ```'#'``` 组成。我们希望按下述规则将 ```s``` 映射为一些小写英文字符：
* 字符（```'a'``` - ```'i'```）分别用（```'1'``` - ```'9'```）表示。
* 字符（```'j'``` - ```'z'```）分别用（```'10#'``` - ```'26#'```）表示。

返回映射之后形成的新字符串。

题目数据保证映射始终唯一。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "10#11#12"
<strong>输出:</strong> "jkab"
<strong>解释:</strong> "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "1326#"
<strong>输出:</strong> "acz"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "25#"
<strong>输出:</strong> "y"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
<strong>输出:</strong> "abcdefghijklmnopqrstuvwxyz"
</pre>

#### 提示:
* ```1 <= s.length <= 1000```
* ```s[i]``` 只包含数字（```'0'```-```'9'```）和 ```'#'``` 字符。
* ```s``` 是映射始终存在的有效字符串。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def freqAlphabets(self, s: str) -> str:
        ret = ""

        i = 0
        while i < len(s):
            if i + 2 < len(s) and s[i + 2] == '#':
                ret += chr(int(s[i:i + 2]) + 96)
                i += 3
            else:
                ret += chr(int(s[i]) + 96)
                i += 1

        return ret
```
