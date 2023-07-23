# 1078. Bigram 分词
给出第一个词 ```first``` 和第二个词 ```second```，考虑在某些文本 ```text``` 中可能以 "```first second third```" 形式出现的情况，其中 ```second``` 紧随 ```first``` 出现，```third``` 紧随 ```second``` 出现。

对于每种这样的情况，将第三个词 "```third```" 添加到答案中，并返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "alice is a good girl she is a good student", first = "a", second = "good"
<strong>输出:</strong> ["girl","student"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "we will we will rock you", first = "we", second = "will"
<strong>输出:</strong> ["we","rock"]
</pre>

#### 提示:
1. ```1 <= text.length <= 1000```
2. ```text``` 由一些用空格分隔的单词组成，每个单词都由小写英文字母组成
3. ```1 <= first.length, second.length <= 10```
4. ```first``` 和 ```second``` 由小写英文字母组成

## 题解 (Python)

### 1. 线性搜索
```Python3
class Solution:
    def findOcurrences(self, text: str, first: str, second: str) -> List[str]:
        third = []
        words = text.split()

        for i in range(len(words) - 2):
            if words[i] == first and words[i + 1] == second:
                third.append(words[i + 2])

        return third
```
