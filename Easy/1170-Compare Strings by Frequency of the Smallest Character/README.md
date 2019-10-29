# 1170. Compare Strings by Frequency of the Smallest Character
Let's define a function ```f(s)``` over a non-empty string ```s```, which calculates the frequency of the smallest character in ```s```. For example, if ```s = "dcce"``` then ```f(s) = 2``` because the smallest character is ```"c"``` and its frequency is 2.

Now, given string arrays ```queries``` and ```words```, return an integer array ```answer```, where each ```answer[i]``` is the number of words such that ```f(queries[i])``` < ```f(W)```, where ```W``` is a word in ```words```.

#### Example 1:
<pre>
<strong>Input:</strong> queries = ["cbd"], words = ["zaaaz"]
<strong>Output:</strong> [1]
<strong>Explanation:</strong> On the first query we have f("cbd") = 1, f("zaaaz") = 3 so f("cbd") < f("zaaaz").
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> queries = ["bbb","cc"], words = ["a","aa","aaa","aaaa"]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> On the first query only f("bbb") < f("aaaa"). On the second query both f("aaa") and f("aaaa") are both > f("cc").
</pre>

#### Constraints:
* ```1 <= queries.length <= 2000```
* ```1 <= words.length <= 2000```
* ```1 <= queries[i].length, words[i].length <= 10```
* ```queries[i][j]```, ```words[i][j]``` are English lowercase letters.

## Solutions (Python)

### 1. Count
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
