# 1528. Shuffle String
Given a string `s` and an integer array `indices` of the **same length**.

The string `s` will be shuffled such that the character at the `i`<sup>`th`</sup> position moves to `indices[i]` in the shuffled string.

Return *the shuffled string*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/09/q1.jpg)
<pre>
<strong>Input:</strong> s = "codeleet", indices = [4,5,6,7,0,2,1,3]
<strong>Output:</strong> "leetcode"
<strong>Explanation:</strong> As shown, "codeleet" becomes "leetcode" after shuffling.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abc", indices = [0,1,2]
<strong>Output:</strong> "abc"
<strong>Explanation:</strong> After shuffling, each character remains in its position.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aiohn", indices = [3,1,4,2,0]
<strong>Output:</strong> "nihao"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "aaiougrt", indices = [4,0,2,6,7,3,1,5]
<strong>Output:</strong> "arigatou"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "art", indices = [1,0,2]
<strong>Output:</strong> "rat"
</pre>

#### Constraints:
* `s.length == indices.length == n`
* `1 <= n <= 100`
* `s` contains only lower-case English letters.
* `0 <= indices[i] < n`
* All values of `indices` are unique (i.e. `indices` is a permutation of the integers from `0` to `n - 1`).

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def restoreString(self, s: str, indices: List[int]) -> str:
        ret = [''] * len(s)

        for i in range(len(s)):
            ret[indices[i]] = s[i]

        return ''.join(ret)
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} s
# @param {Integer[]} indices
# @return {String}
def restore_string(s, indices)
    ret = [''] * s.length

    for i in 0...s.length
        ret[indices[i]] = s[i]
    end

    return ret.join()
end
```
