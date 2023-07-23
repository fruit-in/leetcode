# 1160. 拼写单词
给你一份『词汇表』（字符串数组） ```words``` 和一张『字母表』（字符串） ```chars```。

假如你可以用 ```chars``` 中的『字母』（字符）拼写出 ```words``` 中的某个『单词』（字符串），那么我们就认为你掌握了这个单词。

注意：每次拼写时，```chars``` 中的每个字母都只能用一次。

返回词汇表 ```words``` 中你掌握的所有单词的 **长度之和**。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["cat","bt","hat","tree"], chars = "atach"
<strong>输出:</strong> 6
<strong>解释:</strong>
可以形成字符串 "cat" 和 "hat"，所以答案是 3 + 3 = 6。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["hello","world","leetcode"], chars = "welldonehoneyr"
<strong>输出:</strong> 10
<strong>解释:</strong>
可以形成字符串 "hello" 和 "world"，所以答案是 5 + 5 = 10。
</pre>

#### 提示:
1. ```1 <= words.length <= 1000```
2. ```1 <= words[i].length, chars.length <= 100```
3. 所有字符串中都仅包含小写英文字母

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        ret = 0

        cnt = [0] * 26
        for ch in chars:
            cnt[ord(ch) - 97] += 1

        for word in words:
            cnt_copy = cnt[:]
            flag = True

            for ch in word:
                if not cnt_copy[ord(ch) - 97]:
                    flag = False
                    break
                cnt_copy[ord(ch) - 97] -= 1

            if flag:
                ret += len(word)

        return ret
```
