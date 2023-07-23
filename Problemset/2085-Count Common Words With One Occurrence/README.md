# 2085. Count Common Words With One Occurrence
Given two string arrays `words1` and `words2`, return *the number of strings that appear **exactly once** in **each** of the two arrays*.

#### Example 1:
<pre>
<strong>Input:</strong> words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- "leetcode" appears exactly once in each of the two arrays. We count this string.
- "amazing" appears exactly once in each of the two arrays. We count this string.
- "is" appears in each of the two arrays, but there are 2 occurrences of it in words1. We do not count this string.
- "as" appears once in words1, but does not appear in words2. We do not count this string.
Thus, there are 2 strings that appear exactly once in each of the two arrays.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no strings that appear in each of the two arrays.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words1 = ["a","ab"], words2 = ["a","a","a","ab"]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only string that appears exactly once in each of the two arrays is "ab".
</pre>

#### Constraints:
* `1 <= words1.length, words2.length <= 1000`
* `1 <= words1[i].length, words2[j].length <= 30`
* `words1[i]` and `words2[j]` consists only of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countWords(self, words1: List[str], words2: List[str]) -> int:
        count1 = Counter(words1)
        count2 = Counter(words2)

        return sum(1 for k, v in count1.items() if v == 1 and count2[k] == 1)
```
