# 1880. Check if Word Equals Summation of Two Words
The **letter value** of a letter is its position in the alphabet **starting from 0** (i.e. `'a' -> 0`, `'b' -> 1`, `'c' -> 2`, etc.).

The **numerical value** of some string of lowercase English letters `s` is the **concatenation** of the **letter values** of each letter in `s`, which is then **converted** into an integer.
* For example, if `s = "acb"`, we concatenate each letter's letter value, resulting in `"021"`. After converting it, we get `21`.

You are given three strings `firstWord`, `secondWord`, and `targetWord`, each consisting of lowercase English letters `'a'` through `'j'` **inclusive**.

Return `true` *if the **summation** of the **numerical values** of* `firstWord` *and* `secondWord` *equals the **numerical value** of* `targetWord`, *or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> firstWord = "acb", secondWord = "cba", targetWord = "cdb"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The numerical value of firstWord is "acb" -> "021" -> 21.
The numerical value of secondWord is "cba" -> "210" -> 210.
The numerical value of targetWord is "cdb" -> "231" -> 231.
We return true because 21 + 210 == 231.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> firstWord = "aaa", secondWord = "a", targetWord = "aab"
<strong>Output:</strong> false
<strong>Explanation:</strong>
The numerical value of firstWord is "aaa" -> "000" -> 0.
The numerical value of secondWord is "a" -> "0" -> 0.
The numerical value of targetWord is "aab" -> "001" -> 1.
We return false because 0 + 0 != 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> firstWord = "aaa", secondWord = "a", targetWord = "aaaa"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The numerical value of firstWord is "aaa" -> "000" -> 0.
The numerical value of secondWord is "a" -> "0" -> 0.
The numerical value of targetWord is "aaaa" -> "0000" -> 0.
We return true because 0 + 0 == 0.
</pre>

#### Constraints:
* `1 <= firstWord.length, secondWord.length, targetWord.length <= 8`
* `firstWord`, `secondWord`, and `targetWord` consist of lowercase English letters from `'a'` to `'j'` **inclusive**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let value1 = first_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let value2 = second_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);
        let target = target_word
            .bytes()
            .map(|c| c - b'a')
            .fold(0, |acc, x| acc * 10 + x as i32);

        value1 + value2 == target
    }
}
```
