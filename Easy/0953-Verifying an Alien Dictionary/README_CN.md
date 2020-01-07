# 953. 验证外星语词典
某种外星语也使用英文小写字母，但可能顺序 ```order``` 不同。字母表的顺序（```order```）是一些小写字母的排列。

给定一组用外星语书写的单词 ```words```，以及其字母表的顺序 ```order```，只有当给定的单词在这种外星语中按字典序排列时，返回 ```true```；否则，返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
<strong>输出:</strong> true
<strong>输出:</strong> 在该语言的字母表中，'h' 位于 'l' 之前，所以单词序列是按字典序排列的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
<strong>输出:</strong> false
<strong>输出:</strong> 在该语言的字母表中，'d' 位于 'l' 之后，那么 words[0] > words[1]，因此单词序列不是按字典序排列的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
<strong>输出:</strong> false
<strong>输出:</strong> 当前三个字符 "app" 匹配时，第二个字符串相对短一些，然后根据词典编纂规则 "apple" > "app"，因为 'l' > '∅'，其中 '∅' 是空白字符，定义为比任何其他字符都小（<a href="https://baike.baidu.com/item/%E5%AD%97%E5%85%B8%E5%BA%8F">更多信息</a>）。
</pre>

#### 提示:
1. ```1 <= words.length <= 100```
2. ```1 <= words[i].length <= 20```
3. ```order.length == 26```
4. 在 ```words[i]``` 和 ```order``` 中的所有字符都是英文小写字母。

## 题解 (Python)

### 1. 检查相邻单词
```Python
class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        alien_order = [0] * 26
        for i in range(len(order)):
            alien_order[ord(order[i]) - 97] = i

        for i in range(len(words) - 1):
            for j in range(len(words[i])):
                if (j == len(words[i + 1]) or
                    alien_order[ord(words[i][j]) - 97] >
                    alien_order[ord(words[i + 1][j]) - 97]):
                    return False
                elif (alien_order[ord(words[i][j]) - 97] <
                      alien_order[ord(words[i + 1][j]) - 97]):
                    break

        return True
```

### 2. Translate
```Python
class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        alphabet = "abcdefghijklmnopqrtsuvwxyz"

        prev = ""
        for curr in words:
            word = curr.translate(curr.maketrans(order, alphabet))

            if prev > word:
                return False

            prev = word

        return True
```
