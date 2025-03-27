# 2050. 并行课程 III
给你一个整数 `n` ，表示有 `n` 节课，课程编号从 `1` 到 `n` 。同时给你一个二维整数数组 `relations` ，其中 <code>relations[j] = [prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]</code> ，表示课程 <code>prevCourse<sub>j</sub></code> 必须在课程 <code>nextCourse<sub>j</sub></code> 之前 完成（先修课的关系）。同时给你一个下标从 **0** 开始的整数数组 `time` ，其中 `time[i]` 表示完成第 `(i+1)` 门课程需要花费的 **月份** 数。

请你根据以下规则算出完成所有课程所需要的 **最少** 月份数：
* 如果一门课的所有先修课都已经完成，你可以在 **任意** 时间开始这门课程。
* 你可以 **同时** 上 **任意门课程** 。

请你返回完成所有课程所需要的 **最少** 月份数。

**注意：**测试数据保证一定可以完成所有课程（也就是先修课的关系构成一个有向无环图）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/07/ex1.png)
<pre>
<strong>输入:</strong> n = 3, relations = [[1,3],[2,3]], time = [3,2,5]
<strong>输出:</strong> 8
<strong>解释:</strong> 上图展示了输入数据所表示的先修关系图，以及完成每门课程需要花费的时间。
你可以在月份 0 同时开始课程 1 和 2 。
课程 1 花费 3 个月，课程 2 花费 2 个月。
所以，最早开始课程 3 的时间是月份 3 ，完成所有课程所需时间为 3 + 5 = 8 个月。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/07/ex2.png)
<pre>
<strong>输入:</strong> n = 5, relations = [[1,5],[2,5],[3,5],[3,4],[4,5]], time = [1,2,3,4,5]
<strong>输出:</strong> 12
<strong>解释:</strong> 上图展示了输入数据所表示的先修关系图，以及完成每门课程需要花费的时间。
你可以在月份 0 同时开始课程 1 ，2 和 3 。
在月份 1，2 和 3 分别完成这三门课程。
课程 4 需在课程 3 之后开始，也就是 3 个月后。课程 4 在 3 + 4 = 7 月完成。
课程 5 需在课程 1，2，3 和 4 之后开始，也就是在 max(1,2,3,7) = 7 月开始。
所以完成所有课程所需的最少时间为 7 + 5 = 12 个月。
</pre>

#### 提示:
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>0 <= relations.length <= min(n * (n - 1) / 2, 5 * 10<sup>4</sup>)</code>
* `relations[j].length == 2`
* <code>1 <= prevCourse<sub>j</sub>, nextCourse<sub>j</sub> <= n</code>
* <code>prevCourse<sub>j</sub> != nextCourse<sub>j</sub></code>
* 所有的先修课程对 <code>[prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]</code> 都是 **互不相同** 的。
* `time.length == n`
* <code>1 <= time[i] <= 10<sup>4</sup></code>
* 先修课程图是一个有向无环图。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut next_courses = vec![vec![]; n as usize];
        let mut indgree = vec![0; n as usize];
        let mut start = vec![0; n as usize];
        let mut stack = vec![];
        let mut ret = 0;

        for relation in &relations {
            let (prev, next) = (relation[0] as usize - 1, relation[1] as usize - 1);

            next_courses[prev].push(next);
            indgree[next] += 1;
        }

        for i in 0..n as usize {
            if indgree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(prev) = stack.pop() {
            let end = start[prev] + time[prev];

            for &next in &next_courses[prev] {
                indgree[next] -= 1;
                start[next] = start[next].max(end);
                if indgree[next] == 0 {
                    stack.push(next);
                }
            }

            ret = ret.max(end);
        }

        ret
    }
}
```
