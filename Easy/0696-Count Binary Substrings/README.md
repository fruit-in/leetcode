# 696. Count Binary Substrings
Give a string ```s```, count the number of non-empty (contiguous) substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.

Substrings that occur multiple times are counted the number of times they occur.

#### Example 1:
<pre>
<strong>Input:</strong> "00110011"
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".
Notice that some of these substrings repeat and are counted the number of times they occur.
Also, "00110011" is not a valid substring because <strong>all</strong> the 0's (and 1's) are not grouped together.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "10101"
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.
</pre>

#### Note:
* ```s.length``` will be between 1 and 50,000.
* ```s``` will only consist of "0" or "1" characters.

## Solutions (Python)

### 1. Linear Scan
```Python3
class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        prev, curr = 0, 0
        ret = 0

        for i in range(len(s)):
            curr += 1
            if i == len(s) - 1 or s[i] != s[i + 1]:
                ret += min(prev, curr)
                prev, curr = curr, 0

        return ret
```

## Solutions (Ruby)

### 1. Linear Scan
```Ruby
# @param {String} s
# @return {Integer}
def count_binary_substrings(s)
    prev, curr = 0, 0
    ret = 0

    for i in 0...s.length
        curr += 1
        if i == s.length - 1 or s[i] != s[i + 1]
            ret += [prev, curr].min
            prev, curr = curr, 0
        end
    end

    return ret
end
```
