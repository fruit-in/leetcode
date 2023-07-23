# 1170. 比较字符串最小字母出现频次
我们来定义一个函数 ```f(s)```，其中传入参数 ```s``` 是一个非空字符串；该函数的功能是统计 ```s```  中（按字典序比较）最小字母的出现频次。

例如，若 ```s = "dcce"```，那么 ```f(s) = 2```，因为最小的字母是 ```"c"```，它出现了 2 次。

现在，给你两个字符串数组待查表 ```queries``` 和词汇表 ```words```，请你返回一个整数数组 ```answer``` 作为答案，其中每个 ```answer[i]``` 是满足 ```f(queries[i])``` < ```f(W)``` 的词的数目，```W``` 是词汇表 ```words``` 中的词。

#### 示例 1:
<pre>
<strong>输入:</strong> queries = ["cbd"], words = ["zaaaz"]
<strong>输出:</strong> [1]
<strong>解释:</strong> 查询 f("cbd") = 1，而 f("zaaaz") = 3 所以 f("cbd") < f("zaaaz")。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 第一个查询 f("bbb") < f("aaaa")，第二个查询 f("aaa") 和 f("aaaa") 都 > f("cc")。
</pre>

#### 提示:
* ```1 <= queries.length <= 2000```
* ```1 <= words.length <= 2000```
* ```1 <= queries[i].length, words[i].length <= 10```
* ```queries[i][j]```, ```words[i][j]``` 都是小写英文字母

## 题解 (Python)

### 1. 计数
```Python3
class Solution:
    def numSmallerByFrequency(self, queries: List[str], words: List[str]) -> List[int]:
        queries_cnt = [query.count(min(query)) for query in queries]
        words_cnt = [word.count(min(word)) for word in words]
        ret = []

        for q_cnt in queries_cnt:
            ret.append(sum(1 for w_cnt in words_cnt if w_cnt > q_cnt))

        return ret
```
