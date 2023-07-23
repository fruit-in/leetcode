# 2047. Number of Valid Words in a Sentence
A sentence consists of lowercase letters (`'a'` to `'z'`), digits (`'0'` to `'9'`), hyphens (`'-'`), punctuation marks (`'!'`, `'.'`, and `','`), and spaces (`' '`) only. Each sentence can be broken down into **one or more tokens** separated by one or more spaces `' '`.

A token is a valid word if **all three** of the following are true:
* It only contains lowercase letters, hyphens, and/or punctuation (**no** digits).
* There is **at most one** hyphen `'-'`. If present, it **must** be surrounded by lowercase characters (`"a-b"` is valid, but `"-ab"` and `"ab-"` are not valid).
* There is **at most one** punctuation mark. If present, it **must** be at the **end** of the token (`"ab,"`, `"cd!"`, and `"."` are valid, but `"a!b"` and `"c.,"` are not valid).

Examples of valid words include `"a-b."`, `"afad"`, `"ba-c"`, `"a!"`, and `"!"`.

Given a string `sentence`, return *the **number** of valid words in* `sentence`.

#### Example 1:
<pre>
<strong>Input:</strong> sentence = "cat and  dog"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The valid words in the sentence are "cat", "and", and "dog".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentence = "!this  1-s b8d!"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no valid words in the sentence.
"!this" is invalid because it starts with a punctuation mark.
"1-s" and "b8d" are invalid because they contain digits.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> sentence = "alice and  bob are playing stone-game10"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The valid words in the sentence are "alice", "and", "bob", "are", and "playing".
"stone-game10" is invalid because it contains digits.
</pre>

#### Constraints:
* `1 <= sentence.length <= 1000`
* `sentence` only contains lowercase English letters, digits, `' '`, `'-'`, `'!'`, `'.'`, and `','`.
* There will be at least `1` token.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countValidWords(self, sentence: str) -> int:
        return sum(1 for word in sentence.split() if self.is_valid(word))

    def is_valid(self, word: str) -> bool:
        no_hyphen = True

        for i in range(len(word)):
            if word[i].isdigit():
                return False
            elif word[i] == '-':
                if not no_hyphen:
                    return False
                elif i == 0 or i == len(word) - 1:
                    return False
                elif not (word[i - 1].islower() and word[i + 1].islower()):
                    return False
                else:
                    no_hyphen = False
            elif word[i] in "!.," and i != len(word) - 1:
                return False

        return True
```
