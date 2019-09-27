# 299. Bulls and Cows
You are playing the following [Bulls and Cows](https://en.wikipedia.org/wiki/Bulls_and_Cows) game with your friend: You write down a number and ask your friend to guess what the number is. Each time your friend makes a guess, you provide a hint that indicates how many digits in said guess match your secret number exactly in both digit and position (called "bulls") and how many digits match the secret number but locate in the wrong position (called "cows"). Your friend will use successive guesses and hints to eventually derive the secret number.

Write a function to return a hint according to the secret number and friend's guess, use ```A``` to indicate the bulls and ```B``` to indicate the cows.

Please note that both secret number and friend's guess may contain duplicate digits.

#### Example 1:
<pre>
<strong>Input:</strong> secret = "1807", guess = "7810"
<strong>Output:</strong> "1A3B"
<strong>Explanation:</strong> 1 bull and 3 cows. The bull is 8, the cows are 0, 1 and 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> secret = "1123", guess = "0111"
<strong>Output:</strong> "1A1B"
<strong>Explanation:</strong> The 1st 1 in friend's guess is a bull, the 2nd or 3rd 1 is a cow.
</pre>

#### Note:
You may assume that the secret number and your friend's guess only contain digits, and their lengths are always equal.

## Solutions (Python)

### 1. One Pass
```Python3
class Solution:
    def getHint(self, secret: str, guess: str) -> str:
        bulls = 0

        scows = [0] * 10
        gcows = [0] * 10

        for sch, gch in zip(secret, guess):
            if sch == gch:
                bulls += 1
            else:
                scows[int(sch)] += 1
                gcows[int(gch)] += 1

        cows = sum(min(scow, gcow) for scow, gcow in zip(scows, gcows))

        return "%dA%dB" % (bulls, cows)
```
