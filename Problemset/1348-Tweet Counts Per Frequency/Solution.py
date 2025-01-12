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
