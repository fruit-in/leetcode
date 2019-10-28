# 1160. Find Words That Can Be Formed by Characters
You are given an array of strings ```words``` and a string ```chars```.

A string is *good* if it can be formed by characters from ```chars``` (each character can only be used once).

Return the sum of lengths of all good strings in ```words```.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["cat","bt","hat","tree"], chars = "atach"
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["hello","world","leetcode"], chars = "welldonehoneyr"
<strong>Output:</strong> 10
<strong>Explanation:</strong>
The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.
</pre>

#### Note:
1. ```1 <= words.length <= 1000```
2. ```1 <= words[i].length, chars.length <= 100```
3. All strings contain lowercase English letters only.

## Solutions (Python)

### 1. Solution
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
