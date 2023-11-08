# 207. Course Schedule
There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that you **must** take course <code>b<sub>i</sub></code> first if you want to take course <code>a<sub>i</sub></code>.

* For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.

Return `true` if you can finish all courses. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> numCourses = 2, prerequisites = [[1,0]]
<strong>Output:</strong> true
<strong>Explanation:</strong> There are a total of 2 courses to take.
To take course 1 you should have finished course 0. So it is possible.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> numCourses = 2, prerequisites = [[1,0],[0,1]]
<strong>Output:</strong> false
<strong>Explanation:</strong> There are a total of 2 courses to take.
To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
</pre>

#### Constraints:
* `1 <= numCourses <= 2000`
* `0 <= prerequisites.length <= 5000`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses</code>
* All the pairs prerequisites[i] are **unique**.

## Solutions (Rust)

### 1. Solution
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
