# 2456. 最流行的视频创作者
给你两个字符串数组 `creators` 和 `ids` ，和一个整数数组 `views` ，所有数组的长度都是 `n` 。平台上第 `i` 个视频者是 `creator[i]` ，视频分配的 id 是 `ids[i]` ，且播放量为 `views[i]` 。

视频创作者的 **流行度** 是该创作者的 **所有** 视频的播放量的 **总和** 。请找出流行度 **最高** 创作者以及该创作者播放量 **最大** 的视频的 id 。

* 如果存在多个创作者流行度都最高，则需要找出所有符合条件的创作者。
* 如果某个创作者存在多个播放量最高的视频，则只需要找出字典序最小的 `id` 。

返回一个二维字符串数组 `answer` ，其中 <code>answer[i] = [creator<sub>i</sub>, id<sub>i</sub>]</code> 表示 <code>creator<sub>i</sub></code> 的流行度 **最高** 且其最流行的视频 id 是 <code>id<sub>i</sub></code> ，可以按任何顺序返回该结果。

#### 示例 1:
<pre>
<strong>输入:</strong> creators = ["alice","bob","alice","chris"], ids = ["one","two","three","four"], views = [5,10,5,4]
<strong>输出:</strong> [["alice","one"],["bob","two"]]
<strong>解释:</strong>
alice 的流行度是 5 + 5 = 10 。
bob 的流行度是 10 。
chris 的流行度是 4 。
alice 和 bob 是流行度最高的创作者。
bob 播放量最高的视频 id 为 "two" 。
alice 播放量最高的视频 id 是 "one" 和 "three" 。由于 "one" 的字典序比 "three" 更小，所以结果中返回的 id 是 "one" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> creators = ["alice","alice","alice"], ids = ["a","b","c"], views = [1,2,2]
<strong>输出:</strong> [["alice","b"]]
<strong>解释:</strong>
id 为 "b" 和 "c" 的视频都满足播放量最高的条件。
由于 "b" 的字典序比 "c" 更小，所以结果中返回的 id 是 "b" 。
</pre>

#### 提示:
* `n == creators.length == ids.length == views.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= creators[i].length, ids[i].length <= 5`
* `creators[i]` 和 `ids[i]` 仅由小写英文字母组成
* <code>0 <= views[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
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
