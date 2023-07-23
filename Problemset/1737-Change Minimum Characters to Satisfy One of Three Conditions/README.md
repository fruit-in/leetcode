# 1737. Change Minimum Characters to Satisfy One of Three Conditions
You are given two strings `a` and `b` that consist of lowercase letters. In one operation, you can change any character in `a` or `b` to **any lowercase letter**.

Your goal is to satisfy **one** of the following three conditions:

* **Every** letter in `a` is **strictly less** than **every** letter in `b` in the alphabet.
* **Every** letter in `b` is **strictly less** than **every** letter in `a` in the alphabet.
* **Both** `a` and `b` consist of **only one** distinct letter.

Return *the **minimum** number of operations needed to achieve your goal*.

#### Example 1:
<pre>
<strong>Input:</strong> a = "aba", b = "caa"
<strong>Output:</strong> 2
<strong>Explanation:</strong> Consider the best way to make each condition true:
1) Change b to "ccc" in 2 operations, then every letter in a is less than every letter in b.
2) Change a to "bbb" and b to "aaa" in 3 operations, then every letter in b is less than every letter in a.
3) Change a to "aaa" and b to "aaa" in 2 operations, then a and b consist of one distinct letter.
The best way was done in 2 operations (either condition 1 or condition 3).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = "dabadd", b = "cda"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The best way is to make condition 1 true by changing b to "eee".
</pre>

#### Constraints:
* <code>1 <= a.length, b.length <= 10<sup>5</sup></code>
* `a` and `b` consist only of lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut ret = a
            .chars()
            .chain(b.chars())
            .filter(|&chab| chab != 'a')
            .count();

        for ch in 'b'..='z' {
            ret = ret.min(
                a.chars().filter(|&cha| cha >= ch).count()
                    + b.chars().filter(|&chb| chb < ch).count(),
            );
            ret = ret.min(
                a.chars().filter(|&cha| cha < ch).count()
                    + b.chars().filter(|&chb| chb >= ch).count(),
            );
            ret = ret.min(
                a.chars()
                    .chain(b.chars())
                    .filter(|&chab| chab != ch)
                    .count(),
            );
        }

        ret as i32
    }
}
```
