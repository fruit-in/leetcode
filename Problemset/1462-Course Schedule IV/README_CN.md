# 1462. 课程表 IV
你总共需要上 `numCourses` 门课，课程编号依次为 `0` 到 `numCourses-1` 。你会得到一个数组 `prerequisite` ，其中 <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示如果你想选 <code>b<sub>i</sub></code> 课程，你 **必须** 先选 <code>a<sub>i</sub></code> 课程。

* 有的课会有直接的先修课程，比如如果想上课程 `1` ，你必须先上课程 `0` ，那么会以 `[0,1]` 数对的形式给出先修课程数对。

先决条件也可以是 **间接** 的。如果课程 `a` 是课程 `b` 的先决条件，课程 `b` 是课程 `c` 的先决条件，那么课程 `a` 就是课程 `c` 的先决条件。

你也得到一个数组 `queries` ，其中 <code>queries[j] = [u<sub>j</sub>, v<sub>j</sub>]</code>。对于第 `j` 个查询，您应该回答课程 <code>u<sub>j</sub></code> 是否是课程 <code>v<sub>j</sub></code> 的先决条件。

返回一个布尔数组 `answer` ，其中 `answer[j]` 是第 `j` 个查询的答案。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/01/courses4-1-graph.jpg)
<pre>
<strong>输入:</strong> numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
<strong>输出:</strong> [false,true]
<strong>解释:</strong> [1, 0] 数对表示在你上课程 0 之前必须先上课程 1。
课程 0 不是课程 1 的先修课程，但课程 1 是课程 0 的先修课程。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
<strong>输出:</strong> [false,false]
<strong>解释:</strong> 没有先修课程对，所以每门课程之间是独立的。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/05/01/courses4-3-graph.jpg)
<pre>
<strong>输入:</strong> numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
<strong>输出:</strong> [true,true]
</pre>

#### 提示:
* `2 <= numCourses <= 100`
* `0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= numCourses - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 每一对 <code>[a<sub>i</sub>, b<sub>i</sub>]</code> 都 **不同**
* 先修课程图中没有环。
* <code>1 <= queries.length <= 10<sup>4</sup></code>
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= numCourses - 1</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        use std::collections::HashSet;

        let num_courses = num_courses as usize;
        let mut prerequisites_set = HashSet::new();
        let mut is_prerequisite = vec![vec![false; num_courses]; num_courses];
        let mut indegree = vec![0; num_courses];
        let mut stack = vec![];

        for prerequisite in &prerequisites {
            let (a, b) = (prerequisite[0] as usize, prerequisite[1] as usize);

            prerequisites_set.insert((a, b));
            is_prerequisite[a][b] = true;
            indegree[b] += 1;
        }

        for a in 0..num_courses {
            if indegree[a] == 0 {
                stack.push(a);
            }
        }

        while let Some(a) = stack.pop() {
            for b in 0..num_courses {
                if is_prerequisite[a][b] {
                    for c in 0..num_courses {
                        is_prerequisite[c][b] |= is_prerequisite[c][a];
                    }
                }

                if prerequisites_set.contains(&(a, b)) {
                    indegree[b] -= 1;
                    if indegree[b] == 0 {
                        stack.push(b);
                    }
                }
            }
        }

        queries
            .iter()
            .map(|query| is_prerequisite[query[0] as usize][query[1] as usize])
            .collect()
    }
}
```
