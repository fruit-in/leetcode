# 804. Unique Morse Code Words
International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows: <code>"a"</code> maps to <code>".-"</code>, <code>"b"</code> maps to <code>"-..."</code>, <code>"c"</code> maps to <code>"-.-."</code>, and so on.

For convenience, the full table for the 26 letters of the English alphabet is given below:
<pre>
[".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
</pre>

Now, given a list of words, each word can be written as a concatenation of the Morse code of each letter. For example, "cba" can be written as "-.-..--...", (which is the concatenation "-.-." + "-..." + ".-"). We'll call such a concatenation, the transformation of a word.

Return the number of different transformations among all words we have.

#### Example:
<pre>
<strong>Input:</strong> words = ["gin", "zen", "gig", "msg"]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The transformation of each word is:
"gin" -> "--...-."
"zen" -> "--...-."
"gig" -> "--...--."
"msg" -> "--...--."

There are 2 different transformations, "--...-." and "--...--.".
</pre>

#### Note:
* The length of <code>words</code> will be at most <code>100</code>.
* Each <code>words[i]</code> will have length in range <code>[1, 12]</code>.
* <code>words[i]</code> will only consist of lowercase letters.

## Solutions (Python)

### 1. Set
```Python3
class Solution:
    def uniqueMorseRepresentations(self, words: List[str]) -> int:
        morse = [".-","-...","-.-.","-..",".","..-.","--.",
                 "....","..",".---","-.-",".-..","--","-.",
                 "---",".--.","--.-",".-.","...","-","..-",
                 "...-",".--","-..-","-.--","--.."]
        codeset = {''.join(morse[ord(c) - ord('a')] for c in word) for word in words}
        return len(codeset)
```
