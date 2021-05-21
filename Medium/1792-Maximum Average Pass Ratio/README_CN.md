# 1792. 最大平均通过率
一所学校里有一些班级，每个班级里有一些学生，现在每个班都会进行一场期末考试。给你一个二维数组 `classes` ，其中 <code>classes[i] = [pass<sub>i</sub>, total<sub>i</sub>]</code> ，表示你提前知道了第 `i` 个班级总共有 <code>total<sub>i</sub></code> 个学生，其中只有 <code>pass<sub>i</sub></code> 个学生可以通过考试。

给你一个整数 `extraStudents` ，表示额外有 `extraStudents` 个聪明的学生，他们 一定 能通过任何班级的期末考。你需要给这 `extraStudents` 个学生每人都安排一个班级，使得 **所有** 班级的 **平均** 通过率 **最大** 。

一个班级的 **通过率** 等于这个班级通过考试的学生人数除以这个班级的总人数。**平均通过率** 是所有班级的通过率之和除以班级数目。

请你返回在安排这 `extraStudents` 个学生去对应班级后的 **最大** 平均通过率。与标准答案误差范围在 <code>10<sup>-5</sup></code> 以内的结果都会视为正确结果。

#### 示例 1:
<pre>
<strong>输入:</strong> classes = [[1,2],[3,5],[2,2]], extraStudents = 2
<strong>输出:</strong> 0.78333
<strong>解释:</strong> 你可以将额外的两个学生都安排到第一个班级，平均通过率为 (3/4 + 3/5 + 2/2) / 3 = 0.78333 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> classes = [[2,4],[3,9],[4,5],[2,10]], extraStudents = 4
<strong>输出:</strong> 0.53485
</pre>

#### 提示:
* <code>1 <= classes.length <= 10<sup>5</sup></code>
* `classes[i].length == 2`
* <code>1 <= pass<sub>i</sub> <= total<sub>i</sub> <= 10<sup>5</sup></code>
* <code>1 <= extraStudents <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 堆
```Python
import heapq


class Solution:
    def maxAverageRatio(self, classes: List[List[int]], extraStudents: int) -> float:
        def deltaRatio(p: int, t: int) -> float:
            return (p + 1) / (t + 1) - p / t

        heap = [(-deltaRatio(p, t), p, t) for p, t in classes]
        heapq.heapify(heap)

        for _ in range(extraStudents):
            _, p, t = heapq.heappop(heap)
            heapq.heappush(heap, (-deltaRatio(p + 1, t + 1), p + 1, t + 1))

        return sum(p / t for _, p, t in heap) / len(classes)
```
