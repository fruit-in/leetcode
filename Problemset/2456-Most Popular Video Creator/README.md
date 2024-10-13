# 2456. Most Popular Video Creator
You are given two string arrays `creators` and `ids`, and an integer array `views`, all of length `n`. The <code>i<sup>th</sup></code> video on a platform was created by `creators[i]`, has an id of `ids[i]`, and has `views[i]` views.

The **popularity** of a creator is the **sum** of the number of views on **all** of the creator's videos. Find the creator with the **highest** popularity and the id of their **most** viewed video.

* If multiple creators have the highest popularity, find all of them.
* If multiple videos have the highest view count for a creator, find the lexicographically **smallest** id.

Note: It is possible for different videos to have the same `id`, meaning that `id`s do not uniquely identify a video. For example, two videos with the same ID are considered as distinct videos with their own viewcount.

Return a **2D array** of **strings** `answer` where <code>answer[i] = [creators<sub>i</sub>, id<sub>i</sub>]</code> means that <code>creators<sub>i</sub></code> has the **highest** popularity and <code>id<sub>i</sub></code> is the **id** of their most **popular** video. The answer can be returned in any order.

#### Example 1:
<pre>
<strong>Input:</strong> creators = ["alice","bob","alice","chris"], ids = ["one","two","three","four"], views = [5,10,5,4]
<strong>Output:</strong> [["alice","one"],["bob","two"]]
<strong>Explanation:</strong>
The popularity of alice is 5 + 5 = 10.
The popularity of bob is 10.
The popularity of chris is 4.
alice and bob are the most popular creators.
For bob, the video with the highest view count is "two".
For alice, the videos with the highest view count are "one" and "three". Since "one" is lexicographically smaller than "three", it is included in the answer.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> creators = ["alice","alice","alice"], ids = ["a","b","c"], views = [1,2,2]
<strong>Output:</strong> [["alice","b"]]
<strong>Explanation:</strong>
The videos with id "b" and "c" have the highest view count.
Since "b" is lexicographically smaller than "c", it is included in the answer.
</pre>

#### Constraints:
* `n == creators.length == ids.length == views.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= creators[i].length, ids[i].length <= 5`
* `creators[i]` and `ids[i]` consist only of lowercase English letters.
* <code>0 <= views[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def mostPopularCreator(self, creators: List[str], ids: List[str], views: List[int]) -> List[List[str]]:
        popularities = {}
        mostviewvideo = {}
        maxpopularity = 0
        answer = set()

        for i in range(len(ids)):
            video, mostview = mostviewvideo.get(creators[i], ("", -1))
            popularities[creators[i]] = popularities.get(
                creators[i], 0) + views[i]

            if views[i] > mostview or (views[i] == mostview and ids[i] < video):
                mostviewvideo[creators[i]] = (ids[i], views[i])
            if popularities[creators[i]] > maxpopularity:
                maxpopularity = popularities[creators[i]]
                answer.clear()
            if popularities[creators[i]] == maxpopularity:
                answer.add((creators[i], mostviewvideo[creators[i]][0]))

        return [[c, d] for c, d in answer]
```
