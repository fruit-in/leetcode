# 207. 课程表
你这个学期必须选修 `numCourses` 门课程，记为 `0` 到 `numCourses - 1` 。

在选修某些课程之前需要一些先修课程。 先修课程按数组 `prerequisites` 给出，其中 <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示如果要学习课程 <code>a<sub>i</sub></code> 则 **必须** 先学习课程  <code>b<sub>i</sub></code> 。

* 例如，先修课程对 `[0, 1]` 表示：想要学习课程 `0` ，你需要先完成课程 `1` 。

请你判断是否可能完成所有课程的学习？如果可以，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> numCourses = 2, prerequisites = [[1,0]]
<strong>输出:</strong> true
<strong>解释:</strong> 总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> numCourses = 2, prerequisites = [[1,0],[0,1]]
<strong>输出:</strong> false
<strong>解释:</strong> 总共有 2 门课程。学习课程 1 之前，你需要先完成课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。
</pre>

#### 提示:
* `1 <= numCourses <= 2000`
* `0 <= prerequisites.length <= 5000`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses</code>
* `prerequisites[i]` 中的所有课程对 **互不相同**

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut indegrees = vec![0; num_courses as usize];
        let mut follows = HashMap::new();
        let mut courses = vec![];

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
            for a in follows.remove(&b).unwrap_or(vec![]) {
                indegrees[a] -= 1;
                if indegrees[a] == 0 {
                    courses.push(a);
                }
            }
        }

        indegrees.iter().all(|&x| x == 0)
    }
}
```
