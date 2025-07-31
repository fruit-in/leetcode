# 1776. 车队 II
在一条单车道上有 `n` 辆车，它们朝着同样的方向行驶。给你一个长度为 `n` 的数组 `cars` ，其中 <code>cars[i] = [position<sub>i</sub>, speed<sub>i</sub>]</code> ，它表示：
* <code>position<sub>i</sub></code> 是第 `i` 辆车和道路起点之间的距离（单位：米）。题目保证 <code>position<sub>i</sub> < position<sub>i+1</sub></code> 。
* <code>speed<sub>i</sub></code> 是第 `i` 辆车的初始速度（单位：米/秒）。

简单起见，所有车子可以视为在数轴上移动的点。当两辆车占据同一个位置时，我们称它们相遇了。一旦两辆车相遇，它们会合并成一个车队，这个车队里的车有着同样的位置和相同的速度，速度为这个车队里 最慢 一辆车的速度。

请你返回一个数组 `answer` ，其中 `answer[i]` 是第 `i` 辆车与下一辆车相遇的时间（单位：秒），如果这辆车不会与下一辆车相遇，则 `answer[i]` 为 `-1` 。答案精度误差需在 <code>10<sup>-5</sup></code> 以内。

#### 示例 1:
<pre>
<strong>输入:</strong> cars = [[1,2],[2,1],[4,3],[7,2]]
<strong>输出:</strong> [1.00000,-1.00000,3.00000,-1.00000]
<strong>解释:</strong> 经过恰好 1 秒以后，第一辆车会与第二辆车相遇，并形成一个 1 m/s 的车队。经过恰好 3 秒以后，第三辆车会与第四辆车相遇，并形成一个 2 m/s 的车队。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cars = [[3,4],[5,4],[6,3],[9,1]]
<strong>输出:</strong> [2.00000,1.00000,1.50000,-1.00000]
</pre>

#### 提示:
* <code>1 <= cars.length <= 10<sup>5</sup></code>
* <code>1 <= position<sub>i</sub>, speed<sub>i</sub> <= 10<sup>6</sup></code>
* <code>position<sub>i</sub> < position<sub>i+1</sub></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getCollisionTimes(self, cars: List[List[int]]) -> List[float]:
        stack = []
        answer = [-1] * len(cars)

        for i in range(len(cars) - 1, -1, -1):
            while stack and stack[-1][-1] >= cars[i][1]:
                stack.pop()

            while len(stack) > 1 and (stack[-1][0] - cars[i][0]) / (cars[i][1] - stack[-1][1]) > (stack[-2][0] - cars[i][0]) / (cars[i][1] - stack[-2][1]):
                stack.pop()

            if stack:
                answer[i] = (stack[-1][0] - cars[i][0]) / \
                    (cars[i][1] - stack[-1][1])
            stack.append(cars[i])

        return answer
```
