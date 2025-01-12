# 1348. Tweet Counts Per Frequency
A social media company is trying to monitor activity on their site by analyzing the number of tweets that occur in select periods of time. These periods can be partitioned into smaller **time chunks** based on a certain frequency (every **minute**, **hour**, or **day**).

For example, the period `[10, 10000]` (in **seconds**) would be partitioned into the following **time chunks** with these frequencies:

* Every **minute** (60-second chunks): `[10,69]`, `[70,129]`, `[130,189]`, ..., `[9970,10000]`
* Every **hour** (3600-second chunks): `[10,3609]`, `[3610,7209]`, `[7210,10000]`
* Every **day** (86400-second chunks): `[10,10000]`

Notice that the last chunk may be shorter than the specified frequency's chunk size and will always end with the end time of the period (`10000` in the above example).

Design and implement an API to help the company with their analysis.

Implement the `TweetCounts` class:

* `TweetCounts()` Initializes the `TweetCounts` object.
* `void recordTweet(String tweetName, int time)` Stores the `tweetName` at the recorded `time` (in **seconds**).
* `List<Integer> getTweetCountsPerFrequency(String freq, String tweetName, int startTime, int endTime)` Returns a list of integers representing the number of tweets with `tweetName` in each **time chunk** for the given period of time `[startTime, endTime]` (in **seconds**) and frequency `freq`.
    * freq is one of "minute", "hour", or "day" representing a frequency of every **minute**, **hour**, or **day** respectively.

#### Example:
<pre>
<strong>Input:</strong>
["TweetCounts","recordTweet","recordTweet","recordTweet","getTweetCountsPerFrequency","getTweetCountsPerFrequency","recordTweet","getTweetCountsPerFrequency"]
[[],["tweet3",0],["tweet3",60],["tweet3",10],["minute","tweet3",0,59],["minute","tweet3",0,60],["tweet3",120],["hour","tweet3",0,210]]
<strong>Output:</strong>
[null,null,null,null,[2],[2,1],null,[4]]
<strong>Explanation:</strong>
TweetCounts tweetCounts = new TweetCounts();
tweetCounts.recordTweet("tweet3", 0);                              // New tweet "tweet3" at time 0
tweetCounts.recordTweet("tweet3", 60);                             // New tweet "tweet3" at time 60
tweetCounts.recordTweet("tweet3", 10);                             // New tweet "tweet3" at time 10
tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 59); // return [2]; chunk [0,59] had 2 tweets
tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 60); // return [2,1]; chunk [0,59] had 2 tweets, chunk [60,60] had 1 tweet
tweetCounts.recordTweet("tweet3", 120);                            // New tweet "tweet3" at time 120
tweetCounts.getTweetCountsPerFrequency("hour", "tweet3", 0, 210);  // return [4]; chunk [0,210] had 4 tweets
</pre>

#### Constraints:
* <code>0 <= time, startTime, endTime <= 10<sup>9</sup></code>
* <code>0 <= endTime - startTime <= 10<sup>4</sup></code>
* There will be at most <code>10<sup>4</sup></code> calls **in total** to `recordTweet` and `getTweetCountsPerFrequency`.

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class TweetCounts:

    def __init__(self):
        self.sortedtweets = {}

    def recordTweet(self, tweetName: str, time: int) -> None:
        if tweetName not in self.sortedtweets:
            self.sortedtweets[tweetName] = SortedList()
        self.sortedtweets[tweetName].add(time)

    def getTweetCountsPerFrequency(self, freq: str, tweetName: str, startTime: int, endTime: int) -> List[int]:
        freq = {"minute": 60, "hour": 3600, "day": 86400}[freq]
        ret = []

        for start in range(startTime, endTime + 1, freq):
            end = min(start + freq - 1, endTime)
            if tweetName not in self.sortedtweets:
                ret.append(0)
            else:
                i = self.sortedtweets[tweetName].bisect_left(start)
                j = self.sortedtweets[tweetName].bisect_right(end)
                ret.append(j - i)

        return ret


# Your TweetCounts object will be instantiated and called as such:
# obj = TweetCounts()
# obj.recordTweet(tweetName,time)
# param_2 = obj.getTweetCountsPerFrequency(freq,tweetName,startTime,endTime)
```
