# 210. 课程表 II
现在你总共有 `numCourses` 门课需要选，记为 `0` 到 `numCourses - 1`。给你一个数组 `prerequisites` ，其中 <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示在选修课程 <code>a<sub>i</sub></code> 前 **必须** 先选修 <code>b<sub>i</sub></code> 。

* 例如，想要学习课程 `0` ，你需要先完成课程 `1` ，我们用一个匹配来表示：`[0,1]` 。

返回你为了学完所有课程所安排的学习顺序。可能会有多个正确的顺序，你只要返回 **任意一种** 就可以了。如果不可能完成所有课程，返回 **一个空数组** 。

#### 示例 1:
<pre>
<strong>输入:</strong> numCourses = 2, prerequisites = [[1,0]]
<strong>输出:</strong> [0,1]
<strong>解释:</strong> 总共有 2 门课程。要学习课程 1，你需要先完成课程 0。因此，正确的课程顺序为 [0,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
<strong>输出:</strong> [0,2,1,3]
<strong>解释:</strong> 总共有 4 门课程。要学习课程 3，你应该先完成课程 1 和课程 2。并且课程 1 和课程 2 都应该排在课程 0 之后。
因此，一个正确的课程顺序是 [0,1,2,3] 。另一个正确的排序是 [0,2,1,3] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> numCourses = 1, prerequisites = []
<strong>输出:</strong> [0]
</pre>

#### 提示:
* `1 <= numCourses <= 2000`
* `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 所有<code>[a<sub>i</sub>, b<sub>i</sub>]</code> 互不相同

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegrees = vec![0; num_courses as usize];
        let mut follows = HashMap::new();
        let mut courses = vec![];
        let mut ret = vec![];

        for prerequisite in prerequisites {
            indegrees[prerequisite[0] as usize] += 1;
            follows
                .entry(prerequisite[1] as usize)
                .or_insert(vec![])
                .push(prerequisite[0] as usize);
        }

        for b in 0..num_courses as usize {
            if indegrees[b] == 0 {
                courses.push(b);
            }
        }

        while let Some(b) = courses.pop() {
            ret.push(b as i32);

            for a in follows.remove(&b).unwrap_or(vec![]) {
                indegrees[a] -= 1;
                if indegrees[a] == 0 {
                    courses.push(a);
                }
            }
        }

        if ret.len() < num_courses as usize {
            ret.clear();
        }

        ret
    }
}
```
