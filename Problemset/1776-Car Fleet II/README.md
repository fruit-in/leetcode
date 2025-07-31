# 1776. Car Fleet II
There are `n` cars traveling at different speeds in the same direction along a one-lane road. You are given an array `cars` of length `n`, where <code>cars[i] = [position<sub>i</sub>, speed<sub>i</sub>]</code> represents:
* <code>position<sub>i</sub></code> is the distance between the <code>i<sup>th</sup></code> car and the beginning of the road in meters. It is guaranteed that <code>position<sub>i</sub> < position<sub>i+1</sub></code>.
* <code>speed<sub>i</sub></code> is the initial speed of the <code>i<sup>th</sup></code> car in meters per second.

For simplicity, cars can be considered as points moving along the number line. Two cars collide when they occupy the same position. Once a car collides with another car, they unite and form a single car fleet. The cars in the formed fleet will have the same position and the same speed, which is the initial speed of the **slowest** car in the fleet.

Return an array `answer`, where `answer[i]` is the time, in seconds, at which the <code>i<sup>th</sup></code> car collides with the next car, or `-1` if the car does not collide with the next car. Answers within <code>10<sup>-5</sup></code> of the actual answers are accepted.

#### Example 1:
<pre>
<strong>Input:</strong> cars = [[1,2],[2,1],[4,3],[7,2]]
<strong>Output:</strong> [1.00000,-1.00000,3.00000,-1.00000]
<strong>Explanation:</strong> After exactly one second, the first car will collide with the second car, and form a car fleet with speed 1 m/s. After exactly 3 seconds, the third car will collide with the fourth car, and form a car fleet with speed 2 m/s.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cars = [[3,4],[5,4],[6,3],[9,1]]
<strong>Output:</strong> [2.00000,1.00000,1.50000,-1.00000]
</pre>

#### Constraints:
* <code>1 <= cars.length <= 10<sup>5</sup></code>
* <code>1 <= position<sub>i</sub>, speed<sub>i</sub> <= 10<sup>6</sup></code>
* <code>position<sub>i</sub> < position<sub>i+1</sub></code>

## Solutions (Python)

### 1. Solution
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
