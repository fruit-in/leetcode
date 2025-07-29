# 843. Guess the Word
You are given an array of unique strings `words` where `words[i]` is six letters long. One word of `words` was chosen as a secret word.

You are also given the helper object `Master`. You may call `Master.guess(word)` where `word` is a six-letter-long string, and it must be from `words`. `Master.guess(word)` returns:
* `-1` if `word` is not from `words`, or
* an integer representing the number of exact matches (value and position) of your guess to the secret word.

There is a parameter `allowedGuesses` for each test case where `allowedGuesses` is the maximum number of times you can call `Master.guess(word)`.

For each test case, you should call `Master.guess` with the secret word without exceeding the maximum number of allowed guesses. You will get:
* `"Either you took too many guesses, or you did not find the secret word."` if you called `Master.guess` more than `allowedGuesses` times or if you did not call `Master.guess` with the secret word, or
* `"You guessed the secret word correctly."` if you called `Master.guess` with the secret word with the number of calls to `Master.guess` less than or equal to `allowedGuesses`.

The test cases are generated such that you can guess the secret word with a reasonable strategy (other than using the bruteforce method).

#### Example 1:
<pre>
<strong>Input:</strong> secret = "acckzz", words = ["acckzz","ccbazz","eiowzz","abcczz"], allowedGuesses = 10
<strong>Output:</strong> You guessed the secret word correctly.
<strong>Explanation:</strong>
master.guess("aaaaaa") returns -1, because "aaaaaa" is not in wordlist.
master.guess("acckzz") returns 6, because "acckzz" is secret and has all 6 matches.
master.guess("ccbazz") returns 3, because "ccbazz" has 3 matches.
master.guess("eiowzz") returns 2, because "eiowzz" has 2 matches.
master.guess("abcczz") returns 4, because "abcczz" has 4 matches.
We made 5 calls to master.guess, and one of them was the secret, so we pass the test case.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> secret = "hamada", words = ["hamada","khaled"], allowedGuesses = 10
<strong>Output:</strong> You guessed the secret word correctly.
<strong>Explanation:</strong> Since there are two words, you can guess both.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `words[i].length == 6`
* `words[i]` consist of lowercase English letters.
* All the strings of `wordlist` are **unique**.
* `secret` exists in `words`.
* `10 <= allowedGuesses <= 30`

## Solutions (Python)

### 1. Solution
```Python
# """
# This is Master's API interface.
# You should not implement it, or speculate about its implementation
# """
# class Master:
#     def guess(self, word: str) -> int:

class Solution:
    def findSecretWord(self, words: List[str], master: 'Master') -> None:
        while words:
            buckets = {}
            minmax = inf
            word = words[0]

            for word0 in words:
                buckets[word0] = [[] for _ in range(7)]
                for word1 in words:
                    if word1 != word0:
                        buckets[word0][sum(word0[i] == word1[i]
                                           for i in range(6))].append(word1)

                maxcount = max(len(buckets[word0][i]) for i in range(6))
                if maxcount < minmax:
                    minmax = maxcount
                    word = word0

            matches = master.guess(word)
            words = buckets[word][matches]
```
