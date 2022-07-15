# 2325. Decode the Message
You are given the strings `key` and `message`, which represent a cipher key and a secret message, respectively. The steps to decode `message` are as follows:
1. Use the **first** appearance of all 26 lowercase English letters in `key` as the **order** of the substitution table.
2. Align the substitution table with the regular English alphabet.
3. Each letter in `message` is then **substituted** using the table.
4. Spaces `' '` are transformed to themselves.

* For example, given `key = "happy boy"` (actual key would have **at least one** instance of each letter in the alphabet), we have the partial substitution table of (`'h' -> 'a'`, `'a' -> 'b'`, `'p' -> 'c'`, `'y' -> 'd'`, `'b' -> 'e'`, `'o' -> 'f'`).

Return *the decoded message*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/05/08/ex1new4.jpg)
<pre>
<strong>Input:</strong> key = "the quick brown fox jumps over the lazy dog", message = "vkbs bs t suepuv"
<strong>Output:</strong> "this is a secret"
<strong>Explanation:</strong> The diagram above shows the substitution table.
It is obtained by taking the first appearance of each letter in "the quick brown fox jumps over the lazy dog".
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/05/08/ex2new.jpg)
<pre>
<strong>Input:</strong> key = "eljuxhpwnyrdgtqkviszcfmabo", message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb"
<strong>Output:</strong> "the five boxing wizards jump quickly"
<strong>Explanation:</strong> The diagram above shows the substitution table.
It is obtained by taking the first appearance of each letter in "eljuxhpwnyrdgtqkviszcfmabo".
</pre>

#### Constraints:
* `26 <= key.length <= 2000`
* `key` consists of lowercase English letters and `' '`.
* `key` contains every letter in the English alphabet (`'a'` to `'z'`) **at least once**.
* `1 <= message.length <= 2000`
* `message` consists of lowercase English letters and `' '`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def decodeMessage(self, key: str, message: str) -> str:
        table = {ord(' '): ord(' ')}

        for c in key:
            c = ord(c)
            if c not in table:
                table[c] = ord('a') + len(table) - 1

        return message.translate(table)
```
