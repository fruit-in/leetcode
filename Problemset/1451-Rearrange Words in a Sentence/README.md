# 1451. Rearrange Words in a Sentence
Given a sentence `text` (A *sentence* is a string of space-separated words) in the following format:
* First letter is in upper case.
* Each word in `text` are separated by a single space.

Your task is to rearrange the words in text such that all words are rearranged in an increasing order of their lengths. If two words have the same length, arrange them in their original order.

Return the new text following the format shown above.

#### Example 1:
<pre>
<b>Input:</b> text = "Leetcode is cool"
<b>Output:</b> "Is cool leetcode"
<b>Explanation:</b> There are 3 words, "Leetcode" of length 8, "is" of length 2 and "cool" of length 4.
Output is ordered by length and the new first word starts with capital letter.
</pre>

#### Example 2:
<pre>
<b>Input:</b> text = "Keep calm and code on"
<b>Output:</b> "On and keep calm code"
<b>Explanation:</b> Output is ordered as follows:
"On" 2 letters.
"and" 3 letters.
"keep" 4 letters in case of tie order by position in original text.
"calm" 4 letters.
"code" 4 letters.
</pre>

#### Example 3:
<pre>
<b>Input:</b> text = "To be or not to be"
<b>Output:</b> "To be or to be not"
</pre>

#### Constraints:
* `text` begins with a capital letter and then contains lowercase letters and single space between words.
* `1 <= text.length <= 10^5`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} text
# @return {String}
def arrange_words(text)
    words = text.downcase.split(' ')
    words.sort_by! {|word| word.length}
    words[0].capitalize!

    return words.join(' ')
end
```
