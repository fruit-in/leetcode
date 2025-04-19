# 591. 标签验证器
给定一个表示代码片段的字符串，你需要实现一个验证器来解析这段代码，并返回它是否合法。合法的代码片段需要遵守以下的所有规则：
1. 代码必须被**合法的闭合标签**包围。否则，代码是无效的。
2. **闭合标签**（不一定合法）要严格符合格式：`<TAG_NAME>TAG_CONTENT</TAG_NAME>`。其中，`<TAG_NAME>`是起始标签，`</TAG_NAME>`是结束标签。起始和结束标签中的 TAG_NAME 应当相同。当且仅当 TAG_NAME 和 TAG_CONTENT 都是合法的，闭合标签才是**合法的**。
3. **合法的** `TAG_NAME` 仅含有**大写字母**，长度在范围 [1,9] 之间。否则，该 `TAG_NAME` 是**不合法的**。
4. **合法的** `TAG_CONTENT` 可以包含其他**合法的闭合标签**，**cdata** （请参考规则7）和任意字符（注意参考规则1）除了不匹配的`<`、不匹配的起始和结束标签、不匹配的或带有不合法 TAG_NAME 的闭合标签。否则，`TAG_CONTENT` 是**不合法的**。
5. 一个起始标签，如果没有具有相同 TAG_NAME 的结束标签与之匹配，是不合法的。反之亦然。不过，你也需要考虑标签嵌套的问题。
6. 一个`<`，如果你找不到一个后续的`>`与之匹配，是不合法的。并且当你找到一个`<`或`</`时，所有直到下一个`>`的前的字符，都应当被解析为 TAG_NAME（不一定合法）。
7. cdata 有如下格式：`<![CDATA[CDATA_CONTENT]]>`。`CDATA_CONTENT` 的范围被定义成 `<![CDATA[` 和**后续的第一个** `]]>`之间的字符。
8. `CDATA_CONTENT` 可以包含**任意字符**。cdata 的功能是阻止验证器解析`CDATA_CONTENT`，所以即使其中有一些字符可以被解析为标签（无论合法还是不合法），也应该将它们视为**常规字符**。

#### 合法代码的例子:
<pre>
<strong>输入:</strong> "<DIV>This is the first line <![CDATA[<div>]]></DIV>"
<strong>输出:</strong> True
<strong>解释:</strong>
代码被包含在了闭合的标签内： <DIV> 和 </DIV> 。
TAG_NAME 是合法的，TAG_CONTENT 包含了一些字符和 cdata 。
即使 CDATA_CONTENT 含有不匹配的起始标签和不合法的 TAG_NAME，它应该被视为普通的文本，而不是标签。
所以 TAG_CONTENT 是合法的，因此代码是合法的。最终返回True。

<strong>输入:</strong> "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"
<strong>输出:</strong> True
<strong>解释:</strong>
我们首先将代码分割为： start_tag|tag_content|end_tag 。
start_tag -> "<DIV>"
end_tag -> "</DIV>"
tag_content 也可被分割为： text1|cdata|text2 。
text1 -> ">>  ![cdata[]] "
cdata -> "<![CDATA[<div>]>]]>" ，其中 CDATA_CONTENT 为 "<div>]>"
text2 -> "]]>>]"
start_tag 不是 "<DIV>>>" 的原因参照规则 6 。
cdata 不是 "<![CDATA[<div>]>]]>]]>" 的原因参照规则 7 。
</pre>

#### 不合法代码的例子:
<pre>
<strong>输入:</strong> "<A>  <B> </A>   </B>"
<strong>输出:</strong> False
<strong>解释:</strong> 不合法。如果 "<A>" 是闭合的，那么 "<B>" 一定是不匹配的，反之亦然。
<strong>输入:</strong> "<DIV>  div tag is not closed  <DIV>"
<strong>输出:</strong> False
<strong>输入:</strong> "<DIV>  unmatched <  </DIV>"
<strong>输出:</strong> False
<strong>输入:</strong> "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>"
<strong>输出:</strong> False
<strong>输入:</strong> "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
<strong>输出:</strong> False
<strong>输入:</strong> "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>"
<strong>输出:</strong> False
</pre>

#### 注意:
1. 为简明起见，你可以假设输入的代码（包括提到的**任意字符**）只包含`数字`, `字母`, `'<'`,`'>'`,`'/'`,`'!'`,`'['`,`']'`和`' '`。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def isValid(self, code: str) -> bool:
        cdata = False
        tagstack = []
        i = 0

        while i < len(code):
            if cdata:
                if code[i:i + 3] == "]]>":
                    cdata = False
                    i += 2
            elif tagstack == [] and (code[i] != '<' or code[i:i + 2] in "</<!"):
                return False
            elif code[i:i + 9] == "<![CDATA[":
                cdata = True
                i += 8
            elif code[i:i + 2] == "</":
                for j in range(i + 2, i + 13):
                    if j >= len(code) or j == i + 12 or (j == i + 2 and code[j] == '>'):
                        return False
                    elif code[j] == '>':
                        if tagstack.pop() != code[i + 2:j]:
                            return False
                        if tagstack == [] and j != len(code) - 1:
                            return False
                        i = j
                        break
                    elif not code[j].isupper():
                        return False
            elif code[i] == '<':
                for j in range(i + 1, i + 12):
                    if j >= len(code) or j == i + 11 or (j == i + 1 and code[j] == '>'):
                        return False
                    elif code[j] == '>':
                        tagstack.append(code[i + 1:j])
                        i = j
                        break
                    elif not code[j].isupper():
                        return False

            i += 1

        return tagstack == []
```
