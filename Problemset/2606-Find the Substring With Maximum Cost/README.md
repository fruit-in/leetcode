# 2606. Find the Substring With Maximum Cost
You are given a string `s`, a string `chars` of **distinct** characters and an integer array `vals` of the same length as `chars`.

The **cost of the substring** is the sum of the values of each character in the substring. The cost of an empty string is considered `0`.

The **value of the character** is defined in the following way:

* If the character is not in the string `chars`, then its value is its corresponding position (**1-indexed**) in the alphabet.
    * For example, the value of `'a'` is `1`, the value of `'b'` is `2`, and so on. The value of `'z'` is `26`.
* Otherwise, assuming `i` is the index where the character occurs in the string `chars`, then its value is `vals[i]`.

Return *the maximum cost among all substrings of the string* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "adaa", chars = "d", vals = [-1000]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The value of the characters "a" and "d" is 1 and -1000 respectively.
The substring with the maximum cost is "aa" and its cost is 1 + 1 = 2.
It can be proven that 2 is the maximum cost.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abc", chars = "abc", vals = [-1,-1,-1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The value of the characters "a", "b" and "c" is -1, -1, and -1 respectively.
The substring with the maximum cost is the empty substring "" and its cost is 0.
It can be proven that 0 is the maximum cost.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consist of lowercase English letters.
* `1 <= chars.length <= 26`
* `chars` consist of **distinct** lowercase English letters.
* `vals.length == chars.length`
* `-1000 <= vals[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut char_vals = (1..=26).collect::<Vec<_>>();
        let mut prefix_sum = 0;
        let mut min_sum = 0;
        let mut ret = 0;

        for (i, c) in chars.bytes().enumerate() {
            char_vals[(c - b'a') as usize] = vals[i];
        }

        for c in s.bytes() {
            prefix_sum += char_vals[(c - b'a') as usize];
            min_sum = min_sum.min(prefix_sum);
            ret = ret.max(prefix_sum - min_sum);
        }

        ret
    }
}
```
