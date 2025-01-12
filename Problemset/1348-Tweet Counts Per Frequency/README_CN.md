# 1348. 推文计数
一家社交媒体公司正试图通过分析特定时间段内出现的推文数量来监控其网站上的活动。这些时间段可以根据特定的频率（ **每分钟** 、**每小时** 或 **每一天** ）划分为更小的 **时间段** 。

例如，周期 `[10,10000]` （以 **秒** 为单位）将被划分为以下频率的 **时间块** :

* 每 **分钟** (60秒 块)： `[10,69]`, `[70,129]`, `[130,189]`, ..., `[9970,10000]`
* 每 **小时** (3600秒 块)：`[10,3609]`, `[3610,7209]`, `[7210,10000]`
* 每 **天** (86400秒 块)： `[10,10000]`

注意，最后一个块可能比指定频率的块大小更短，并且总是以时间段的结束时间结束(在上面的示例中为 `10000` )。

设计和实现一个API来帮助公司进行分析。

实现 `TweetCounts` 类:

* `TweetCounts()` 初始化 `TweetCounts` 对象。
* 存储记录时间的 `tweetName` (以秒为单位)。
* `List<integer> getTweetCountsPerFrequency(String freq, String tweetName, int startTime, int endTime)` 返回一个整数列表，表示给定时间 `[startTime, endTime]` （单位秒）和频率频率中，每个 **时间块** 中带有 `tweetName` 的 `tweet` 的数量。
    * `freq` 是 `“minute”` 、 `“hour”` 或 `“day”` 中的一个，分别表示 **每分钟** 、 **每小时** 或 **每一天** 的频率。

#### 示例:
<pre>
<strong>输入:</strong>
["TweetCounts","recordTweet","recordTweet","recordTweet","getTweetCountsPerFrequency","getTweetCountsPerFrequency","recordTweet","getTweetCountsPerFrequency"]
[[],["tweet3",0],["tweet3",60],["tweet3",10],["minute","tweet3",0,59],["minute","tweet3",0,60],["tweet3",120],["hour","tweet3",0,210]]
<strong>输出:</strong>
[null,null,null,null,[2],[2,1],null,[4]]
<strong>解释:</strong>
TweetCounts tweetCounts = new TweetCounts();
tweetCounts.recordTweet("tweet3", 0);                              // New tweet "tweet3" at time 0
tweetCounts.recordTweet("tweet3", 60);                             // New tweet "tweet3" at time 60
tweetCounts.recordTweet("tweet3", 10);                             // New tweet "tweet3" at time 10
tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 59); // return [2]; chunk [0,59] had 2 tweets
tweetCounts.getTweetCountsPerFrequency("minute", "tweet3", 0, 60); // return [2,1]; chunk [0,59] had 2 tweets, chunk [60,60] had 1 tweet
tweetCounts.recordTweet("tweet3", 120);                            // New tweet "tweet3" at time 120
tweetCounts.getTweetCountsPerFrequency("hour", "tweet3", 0, 210);  // return [4]; chunk [0,210] had 4 tweets
</pre>

#### 提示:
* <code>0 <= time, startTime, endTime <= 10<sup>9</sup></code>
* <code>0 <= endTime - startTime <= 10<sup>4</sup></code>
* `recordTweet` 和 `getTweetCountsPerFrequency`，最多有 <code>10<sup>4</sup></code> 次操作。

## 题解 (Python)

### 1. 题解
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
