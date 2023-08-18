# 1807. Evaluate the Bracket Pairs of a String
You are given a string `s` that contains some bracket pairs, with each pair containing a **non-empty** key.

* For example, in the string `"(name)is(age)yearsold"`, there are **two** bracket pairs that contain the keys `"name"` and `"age"`.

You know the values of a wide range of keys. This is represented by a 2D string array `knowledge` where each <code>knowledge[i] = [key<sub>i</sub>, value<sub>i</sub>]</code> indicates that key <code>key<sub>i</sub></code> has a value of <code>value<sub>i</sub></code>.

You are tasked to evaluate **all** of the bracket pairs. When you evaluate a bracket pair that contains some key <code>key<sub>i</sub></code>, you will:

* Replace <code>key<sub>i</sub></code> and the bracket pair with the key's corresponding <code>value<sub>i</sub></code>.
* If you do not know the value of the key, you will replace <code>key<sub>i</sub></code> and the bracket pair with a question mark `"?"` (without the quotation marks).

Each key will appear at most once in your `knowledge`. There will not be any nested brackets in `s`.

Return *the resulting string after evaluating **all** of the bracket pairs*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "(name)is(age)yearsold", knowledge = [["name","bob"],["age","two"]]
<strong>Output:</strong> "bobistwoyearsold"
<strong>Explanation:</strong>
The key "name" has a value of "bob", so replace "(name)" with "bob".
The key "age" has a value of "two", so replace "(age)" with "two".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "hi(name)", knowledge = [["a","b"]]
<strong>Output:</strong> "hi?"
<strong>Explanation:</strong> As you do not know the value of the key "name", replace "(name)" with "?".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "(a)(a)(a)aaa", knowledge = [["a","yes"]]
<strong>Output:</strong> "yesyesyesaaa"
<strong>Explanation:</strong> The same key can appear multiple times.
The key "a" has a value of "yes", so replace all occurrences of "(a)" with "yes".
Notice that the "a"s not in a bracket pair are not evaluated.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* <code>0 <= knowledge.length <= 10<sup>5</sup></code>
* `knowledge[i].length == 2`
* <code>1 <= key<sub>i</sub>.length, value<sub>i</sub>.length <= 10</code>
* `s` consists of lowercase English letters and round brackets `'('` and `')'`.
* Every open bracket `'('` in `s` will have a corresponding close bracket `')'`.
* The key in each bracket pair of `s` will be non-empty.
* There will not be any nested bracket pairs in `s`.
* <code>key<sub>i</sub></code> and <code>value<sub>i</sub></code> consist of lowercase English letters.
* Each <code>key<sub>i</sub></code> in `knowledge` is unique.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        knowledge = dict(knowledge)
        key = ""
        ret = ""

        for ch in s:
            if ch == ')':
                ret += knowledge.get(key[1:], "?")
                key = ""
            elif key != "" or ch == '(':
                key += ch
            else:
                ret += ch

        return ret
```
