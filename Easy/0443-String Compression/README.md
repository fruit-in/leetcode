# 443. String Compression
Given an array of characters, compress it **[in-place](https://en.wikipedia.org/wiki/In-place_algorithm)**.

The length after compression must always be smaller than or equal to the original array.

Every element of the array should be a **character** (not int) of length 1.

After you are done **modifying the input array [in-place](https://en.wikipedia.org/wiki/In-place_algorithm)**, return the new length of the array.


#### Follow up:
Could you solve it using only O(1) extra space?

#### Example 1:
<pre>
<strong>Input:</strong>
["a","a","b","b","c","c","c"]
<strong>Output:</strong>
Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
<strong>Explanation:</strong>
"aa" is replaced by "a2". "bb" is replaced by "b2". "ccc" is replaced by "c3".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
["a"]
<strong>Output:</strong>
Return 1, and the first 1 characters of the input array should be: ["a"]
<strong>Explanation:</strong>
Nothing is replaced.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong>
["a","b","b","b","b","b","b","b","b","b","b","b","b"]
<strong>Output:</strong>
Return 4, and the first 4 characters of the input array should be: ["a","b","1","2"].
<strong>Explanation:</strong>
Since the character "a" does not repeat, it is not compressed. "bbbbbbbbbbbb" is replaced by "b12".
Notice each digit has it's own entry in the array.
</pre>

#### Note:
1. All characters have an ASCII value in ```[35, 126]```.
2. ```1 <= len(chars) <= 1000```.

## Solutions (Python)

### 1. Two Pointers
```Python3
class Solution:
    def compress(self, chars: List[str]) -> int:
        r, w_char, w_pos = 0, 0, 0

        for r in range(len(chars)):
            if r == len(chars) - 1 or chars[r] != chars[r + 1]:
                s = chars[w_char] + (str(r - w_char + 1) if r > w_char else "")

                for ch in s:
                    chars[w_pos] = ch
                    w_pos += 1

                w_char = r + 1

        return w_pos
```
