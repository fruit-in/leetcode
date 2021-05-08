# 809. Expressive Words
Sometimes people repeat letters to represent extra feeling, such as "hello" -> "heeellooo", "hi" -> "hiiii".  In these strings like "heeellooo", we have *groups* of adjacent letters that are all the same:  "h", "eee", "ll", "ooo".

For some given string `S`, a query word is *stretchy* if it can be made to be equal to `S` by any number of applications of the following *extension* operation: choose a group consisting of characters `c`, and add some number of characters `c` to the group so that the size of the group is 3 or more.

For example, starting with "hello", we could do an extension on the group "o" to get "hellooo", but we cannot get "helloo" since the group "oo" has size less than 3.  Also, we could do another extension like "ll" -> "lllll" to get "helllllooo".  If `S = "helllllooo"`, then the query word "hello" would be stretchy because of these two extension operations: `query = "hello" -> "hellooo" -> "helllllooo" = S`.

Given a list of query words, return the number of words that are stretchy.

#### Example:
<pre>
<strong>Input:</strong>
S = "heeellooo"
words = ["hello", "hi", "helo"]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can extend "e" and "o" in the word "hello" to get "heeellooo".
We can't extend "helo" to get "heeellooo" because the group "ll" is not size 3 or more.
</pre>

#### Constraints:
* `0 <= len(S) <= 100`.
* `0 <= len(words) <= 100`.
* `0 <= len(words[i]) <= 100`.
* `S` and all words in `words` consist only of lowercase letters

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s = Self::group(&s);

        words
            .iter()
            .map(|w| Self::group(w))
            .filter(|w| {
                s.len() == w.len()
                    && s.iter().zip(w.iter()).all(|(&(c0, x), &(c1, y))| {
                        c0 == c1 && ((y == 1 && x != 2) || (y != 1 && x >= y))
                    })
            })
            .count() as i32
    }

    fn group(s: &str) -> Vec<(char, i32)> {
        let mut ret = vec![];

        for c0 in s.chars() {
            match ret.last_mut() {
                Some((c1, x)) if c0 == *c1 => *x += 1,
                _ => ret.push((c0, 1)),
            }
        }

        ret
    }
}
```
