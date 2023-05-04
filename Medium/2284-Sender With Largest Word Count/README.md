# 2284. Sender With Largest Word Count
You have a chat log of `n` messages. You are given two string arrays `messages` and `senders` where `messages[i]` is a **message** sent by `senders[i]`.

A **message** is list of **words** that are separated by a single space with no leading or trailing spaces. The **word count** of a sender is the total number of **words** sent by the sender. Note that a sender may send more than one message.

Return *the sender with the **largest** word count*. If there is more than one sender with the largest word count, return *the one with the **lexicographically largest** name*.

#### Note:

* Uppercase letters come before lowercase letters in lexicographical order.
* `"Alice"` and `"alice"` are distinct.

#### Example 1:
<pre>
<strong>Input:</strong> messages = ["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"], senders = ["Alice","userTwo","userThree","Alice"]
<strong>Output:</strong> "Alice"
<strong>Explanation:</strong> Alice sends a total of 2 + 3 = 5 words.
userTwo sends a total of 2 words.
userThree sends a total of 3 words.
Since Alice has the largest word count, we return "Alice".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> messages = ["How is leetcode for everyone","Leetcode is useful for practice"], senders = ["Bob","Charlie"]
<strong>Output:</strong> "Charlie"
<strong>Explanation:</strong> Bob sends a total of 5 words.
Charlie sends a total of 5 words.
Since there is a tie for the largest word count, we return the sender with the lexicographically larger name, Charlie.
</pre>

#### Constraints:
* `n == messages.length == senders.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* `1 <= messages[i].length <= 100`
* `1 <= senders[i].length <= 10`
* `messages[i]` consists of uppercase and lowercase English letters and `' '`.
* All the words in `messages[i]` are separated by **a single space**.
* `messages[i]` does not have leading or trailing spaces.
* `senders[i]` consists of uppercase and lowercase English letters only.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def largestWordCount(self, messages: List[str], senders: List[str]) -> str:
        wordcount = {}

        for (sender, message) in zip(senders, messages):
            if sender not in wordcount:
                wordcount[sender] = 0
            wordcount[sender] += len(message.split())

        return max(senders, key=lambda s: (wordcount[s], s))
```
