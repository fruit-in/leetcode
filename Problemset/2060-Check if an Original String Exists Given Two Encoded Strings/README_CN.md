# 2060. 同源字符串检测
原字符串由小写字母组成，可以按下述步骤编码：
* 任意将其 **分割** 为由若干 **非空** 子字符串组成的一个 **序列** 。
* 任意选择序列中的一些元素（也可能不选择），然后将这些元素替换为元素各自的长度（作为一个数字型的字符串）。
* 重新 **顺次连接** 序列，得到编码后的字符串。

例如，编码 `"abcdefghijklmnop"` 的一种方法可以描述为：
* 将原字符串分割得到一个序列：`["ab", "cdefghijklmn", "o", "p"]` 。
* 选出其中第二个和第三个元素并分别替换为它们自身的长度。序列变为 `["ab", "12", "1", "p"]` 。
* 重新顺次连接序列中的元素，得到编码后的字符串：`"ab121p"` 。

给你两个编码后的字符串 `s1` 和 `s2` ，由小写英文字母和数字 `1-9` 组成。如果存在能够同时编码得到 `s1` 和 `s2` 原字符串，返回 `true` ；否则，返回 `false`。

**注意：**生成的测试用例满足 `s1` 和 `s2` 中连续数字数不超过 `3` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "internationalization", s2 = "i18n"
<strong>输出:</strong> true
<strong>解释:</strong> "internationalization" 可以作为原字符串
- "internationalization"
  -> 分割：      ["internationalization"]
  -> 不替换任何元素
  -> 连接：      "internationalization"，得到 s1
- "internationalization"
  -> 分割：      ["i", "nternationalizatio", "n"]
  -> 替换：      ["i", "18",                 "n"]
  -> 连接：      "i18n"，得到 s2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "l123e", s2 = "44"
<strong>输出:</strong> true
<strong>解释:</strong> "leetcode" 可以作为原字符串
- "leetcode"
  -> 分割：       ["l", "e", "et", "cod", "e"]
  -> 替换：       ["l", "1", "2",  "3",   "e"]
  -> 连接：       "l123e"，得到 s1
- "leetcode"
  -> 分割：       ["leet", "code"]
  -> 替换：       ["4",    "4"]
  -> 连接：       "44"，得到 s2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "a5b", s2 = "c5b"
<strong>输出:</strong> false
<strong>解释:</strong> 不存在这样的原字符串
- 编码为 s1 的字符串必须以字母 'a' 开头
- 编码为 s2 的字符串必须以字母 'c' 开头
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s1 = "112s", s2 = "g841"
<strong>输出:</strong> true
<strong>解释:</strong> "gaaaaaaaaaaaas" 可以作为原字符串
- "gaaaaaaaaaaaas"
  -> 分割：       ["g", "aaaaaaaaaaaa", "s"]
  -> 替换：       ["1", "12",           "s"]
  -> 连接：       "112s"，得到 s1
- "gaaaaaaaaaaaas"
  -> 分割：       ["g", "aaaaaaaa", "aaaa", "s"]
  -> 替换：       ["g", "8",        "4",    "1"]
  -> 连接         "g841"，得到 s2
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s1 = "ab", s2 = "a2"
<strong>输出:</strong> false
<strong>解释:</strong> 不存在这样的原字符串
- 编码为 s1 的字符串由两个字母组成
- 编码为 s2 的字符串由三个字母组成
</pre>

#### 提示:
* `1 <= s1.length, s2.length <= 40`
* `s1` 和 `s2` 仅由数字 `1-9` 和小写英文字母组成
* `s1` 和 `s2` 中连续数字数不超过 `3`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def possiblyEquals(self, s1: str, s2: str) -> bool:
        @cache
        def isMatchWithSkips(i: int, j: int, diff: int) -> bool:
            if i == len(s1) and j == len(s2):
                return diff == 0
            elif j == len(s2) or diff < 0:
                if diff >= 0 or i == len(s1):
                    return False
                elif s1[i].islower():
                    return isMatchWithSkips(i + 1, j, diff + 1)
                else:
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
            elif i == len(s1) or diff > 0:
                if diff <= 0 or j == len(s2):
                    return False
                elif s2[j].islower():
                    return isMatchWithSkips(i, j + 1, diff - 1)
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False
            else:
                if s1[i].islower() and s2[j].islower():
                    return s1[i] == s2[j] and isMatchWithSkips(i + 1, j + 1, diff)
                elif s1[i].isdigit():
                    for k in range(3):
                        if i + k < len(s1) and s1[i:i + k + 1].isdigit() and isMatchWithSkips(i + k + 1, j, diff + int(s1[i:i + k + 1])):
                            return True
                    return False
                else:
                    for k in range(3):
                        if j + k < len(s2) and s2[j:j + k + 1].isdigit() and isMatchWithSkips(i, j + k + 1, diff - int(s2[j:j + k + 1])):
                            return True
                    return False

        return isMatchWithSkips(0, 0, 0)
```
