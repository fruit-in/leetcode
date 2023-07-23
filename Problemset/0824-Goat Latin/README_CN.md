# 824. 山羊拉丁文
给定一个由空格分割单词的句子 ```S```。每个单词只包含大写或小写字母。

我们要将句子转换为 *“Goat Latin”*（一种类似于 猪拉丁文 - Pig Latin 的虚构语言）。

山羊拉丁文的规则如下：
* 如果单词以元音开头（a, e, i, o, u），在单词后添加```"ma"```。<br>例如，单词```"apple"```变为```"applema"```。
* 如果单词以辅音字母开头（即非元音字母），移除第一个字符并将它放到末尾，之后再添加```"ma"```。<br>例如，单词```"goat"```变为```"oatgma"```。
* 根据单词在句子中的索引，在单词最后添加与索引相同数量的字母```'a'```，索引从1开始。<br>例如，在第一个单词后添加```"a"```，在第二个单词后添加```"aa"```，以此类推。

返回将 ```S``` 转换为山羊拉丁文后的句子。

#### 示例 1:
<pre>
<strong>输入:</strong> "I speak Goat Latin"
<strong>输出:</strong> "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "The quick brown fox jumped over the lazy dog"
<strong>输出:</strong> "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa
ogdmaaaaaaaaaa"
</pre>

#### 说明:
* ```S``` 中仅包含大小写字母和空格。单词间有且仅有一个空格。
* ```1 <= S.length <= 150```。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def toGoatLatin(self, S: str) -> str:
        words = S.split(' ')
        for i, word in enumerate(words):
            if word[0].lower() not in 'aeiou':
                words[i] = word[1:] + word[:1]
            words[i] += 'maa' + 'a' * i
        return ' '.join(words)
```
