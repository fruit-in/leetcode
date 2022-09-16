# 2109. Adding Spaces to a String
You are given a **0-indexed** string `s` and a **0-indexed** integer array `spaces` that describes the indices in the original string where spaces will be added. Each space should be inserted **before** the character at the given index.
* For example, given `s = "EnjoyYourCoffee"` and `spaces = [5, 9]`, we place spaces before `'Y'` and `'C'`, which are at indices `5` and `9` respectively. Thus, we obtain `"Enjoy Your Coffee"`.

Return *the modified string **after** the spaces have been added*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "LeetcodeHelpsMeLearn", spaces = [8,13,15]
<strong>Output:</strong> "Leetcode Helps Me Learn"
<strong>Explanation:</strong>
The indices 8, 13, and 15 correspond to the underlined characters in "LeetcodeHelpsMeLearn".
We then place spaces before those characters.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "icodeinpython", spaces = [1,5,7,9]
<strong>Output:</strong> "i code in py thon"
<strong>Explanation:</strong>
The indices 1, 5, 7, and 9 correspond to the underlined characters in "icodeinpython".
We then place spaces before those characters.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "spacing", spaces = [0,1,2,3,4,5,6]
<strong>Output:</strong> " s p a c i n g"
<strong>Explanation:</strong>
We are also able to place spaces before the first character of the string.
</pre>

#### Constraints:
* <code>1 <= s.length <= 3 * 10<sup>5</sup></code>
* `s` consists only of lowercase and uppercase English letters.
* <code>1 <= spaces.length <= 3 * 10<sup>5</sup></code>
* `0 <= spaces[i] <= s.length - 1`
* All the values of `spaces` are **strictly increasing**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut ret = vec![];

        for i in spaces {
            for j in prev..i as usize {
                ret.push(s[j]);
            }
            ret.push(b' ');
            prev = i as usize;
        }

        for j in prev..s.len() {
            ret.push(s[j]);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
