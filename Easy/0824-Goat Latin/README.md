# 824. Goat Latin
A sentence <code>S</code> is given, composed of words separated by spaces. Each word consists of lowercase and uppercase letters only.

We would like to convert the sentence to "*Goat Latin*" (a made-up language similar to Pig Latin.)

The rules of Goat Latin are as follows:
* If a word begins with a vowel (a, e, i, o, or u), append <code>"ma"</code> to the end of the word.<br>
For example, the word 'apple' becomes 'applema'.

* If a word begins with a consonant (i.e. not a vowel), remove the first letter and append it to the end, then add <code>"ma"</code>.<br>
For example, the word <code>"goat"</code> becomes <code>"oatgma"</code>.

* Add one letter <code>'a'</code> to the end of each word per its word index in the sentence, starting with 1.<br>
For example, the first word gets <code>"a"</code> added to the end, the second word gets <code>"aa"</code> added to the end and so on.

Return the final sentence representing the conversion from <code>S</code> to Goat Latin.

#### Example 1:
<pre>
<strong>Input:</strong> "I speak Goat Latin"
<strong>Output:</strong> "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "The quick brown fox jumped over the lazy dog"
<strong>Output:</strong> "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa
ogdmaaaaaaaaaa"
</pre>

#### Notes:
* <code>S</code> contains only uppercase, lowercase and spaces. Exactly one space between each word.
* <code>1 <= S.length <= 150</code>.

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def toGoatLatin(self, S: str) -> str:
        words = S.split(' ')
        for i, word in enumerate(words):
            if word[0].lower() not in 'aeiou':
                words[i] = word[1:] + word[:1]
            words[i] += 'maa' + 'a' * i
        return ' '.join(words)
```
